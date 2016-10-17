#[macro_use]
extern crate lazy_static;

use std::char::from_u32;
use std::collections::BTreeMap;
use std::iter::Peekable;
use std::str::Chars;

mod entities;

struct Entities {
  entities: BTreeMap<&'static str, &'static str>,
  max_html_length: usize
}

impl Entities {
  fn new() -> Self {
    let entities = entities::entities();
    let max_html_length = entities.keys().map(|x| x.len()).max().unwrap();

    Entities {
      entities: entities,
      max_html_length: max_html_length
    }
  }
}

lazy_static! {
  static ref ENTITIES: Entities = Entities::new();
}

#[derive(Clone, Debug)]
pub enum DecodeError {
  IllFormedEntity(Line, Col),
  UnknownEntity(Line, Col, String),
  EOF
}

type Line = usize;
type Col = usize;

pub fn decode_html_entities(html: &str) -> Result<String, DecodeError> {
  let mut chars = html.chars().peekable(); // iterator over the HTML to decode
  let mut decoded = String::new(); // decoded string
  let mut line = 1; // current line
  let mut col = 1; // current column

  while let Some(c) = chars.next() {
    if c == '&' {
      let unicode_entity = if let Some(&'#') = chars.peek() {
        chars.next();
        try!(parse_entity_numeric(&mut chars, line, &mut col))
      } else {
        let entity = try!(parse_entity_name(&mut chars, line, &mut col));
        let unicode = try!(ENTITIES.entities.get(entity.as_str()).ok_or(DecodeError::UnknownEntity(line, col, entity)));
        (*unicode).to_owned()
      };
      
      decoded += &unicode_entity;
    } else {
      if c == '\n' {
        line += 1;
        col = 1;
      }

      col += 1;
      decoded.push(c);
    }
  }

  Ok(decoded)
}

fn parse_entity_numeric(chars: &mut Peekable<Chars>, line: usize, col: &mut usize) -> Result<String, DecodeError> {
  let c = match chars.peek() {
    Some(&'x') | Some(&'X') => {
      chars.next();
      try!(parse_entity_hex(chars, line, col))
    },
    _ => try!(parse_entity_dec(chars, line, col))
  };

  let mut s = String::new();
  s.push(c);
  Ok(s)
}

fn parse_entity_hex(chars: &mut Peekable<Chars>, line: usize, col: &mut usize) -> Result<char, DecodeError> {
  let num = try!(parse_number(chars, line, col));
  let dec = try!(u32::from_str_radix(&num, 16).map_err(|_| DecodeError::IllFormedEntity(line, *col)));

  from_u32(dec).ok_or(DecodeError::IllFormedEntity(line, *col))
}

fn parse_entity_dec(chars: &mut Peekable<Chars>, line: usize, col: &mut usize) -> Result<char, DecodeError> {
  let num = try!(try!(parse_number(chars, line, col)).parse().map_err(|_| DecodeError::IllFormedEntity(line, *col)));
  from_u32(num).ok_or(DecodeError::IllFormedEntity(line, *col))
}

fn parse_number(chars: &mut Peekable<Chars>, line: usize, col: &mut usize) -> Result<String, DecodeError> {
  let mut hex = String::new();
  let mut l = 0;

  loop {
    if let Some(c) = chars.next() {
      *col += 1;

      if c == ';' {
        break;
      }

      l += 1;

      // abort on long numbers people would try to make to break our code
      if l >= 16 {
        return Err(DecodeError::IllFormedEntity(line, *col));
      }

      hex.push(c);
    } else {
      return Err(DecodeError::EOF)
    }
  }

  Ok(hex)
}

fn parse_entity_name(chars: &mut Peekable<Chars>, line: usize, col: &mut usize) -> Result<String, DecodeError> {
  let mut entity = String::with_capacity(ENTITIES.max_html_length);
  let mut l = 0;

  entity.push('&');
  while l < ENTITIES.max_html_length {
    match chars.next() {
      Some(c) => {
        entity.push(c);

        if c == ';' {
          break;
        }

        l += 1;
        *col += 1;
      },
      None => return Err(DecodeError::EOF)
    }
  }

  if l == ENTITIES.max_html_length {
    Err(DecodeError::IllFormedEntity(line, *col))
  } else {
    Ok(entity)
  }
}
