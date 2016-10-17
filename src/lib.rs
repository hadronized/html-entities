#[macro_use]
extern crate lazy_static;

use std::collections::BTreeMap;
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
  let mut chars = html.chars(); // iterator over the HTML to decode
  let mut decoded = String::new(); // decoded string
  let mut line = 1; // current line
  let mut col = 1; // current column

  while let Some(c) = chars.next() {
    if c == '&' {
      let entity = try!(parse_entity(&mut chars, line, &mut col));
      let unicode_entity = try!(ENTITIES.entities.get(&*entity).ok_or(DecodeError::UnknownEntity(line, col, entity)));
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

fn parse_entity(chars: &mut Chars, line: usize, col: &mut usize) -> Result<String, DecodeError> {
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
