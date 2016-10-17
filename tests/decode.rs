extern crate html_entities;

#[test]
fn by_name() {
  use html_entities::decode_html_entities;
  use std::result::Result;

  let html = "&amp;foobar&lt;&gt;";
  let expected = "&foobar<>";
  let decoded = decode_html_entities(html).unwrap();

  assert_eq!(decoded, expected);
}

#[test]
fn by_dec() {
  use html_entities::decode_html_entities;
  use std::result::Result;

  let html = "&#38;foobar&#60;&#62;";
  let expected = "&foobar<>";
  let decoded = decode_html_entities(html).unwrap();

  assert_eq!(decoded, expected);
}

#[test]
fn by_hex() {
  use html_entities::decode_html_entities;
  use std::result::Result;

  let html = "&#x00026;foobar&#x0003C;&#x0003E;";
  let expected = "&foobar<>";
  let decoded = decode_html_entities(html).unwrap();

  assert_eq!(decoded, expected);
}
