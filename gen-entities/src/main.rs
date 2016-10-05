extern crate serde_json;

use serde_json::de;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;

type Entities = BTreeMap<String, String>;

fn main() {
  let entities: Entities = de::from_reader(File::open("entities.json").unwrap()).unwrap();
  let mut output = File::create("entities.rs").unwrap();

  // header
  let header = r#"use std::collections::BTreeMap;

pub fn entities() -> BTreeMap<&'static str, &'static str> {
  let mut entities = BTreeMap::new();
"#;

  let _ = write!(&mut output, "{}", header);

  for (html, unicode) in entities {
    let _ = write!(&mut output, "  entities.insert({:?}, {:?});\n", html, unicode);
  }

  let end = "  entities\n}\n";

  let _ = write!(&mut output, "{}", end);
}
