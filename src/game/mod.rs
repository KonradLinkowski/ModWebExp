pub mod state;
pub mod command;

use rust_embed::RustEmbed;
use yaml_rust::{YamlLoader};

#[derive(RustEmbed)]
#[folder = "scenarios"]
struct Scenario;

pub fn start() {
  let scenario = Scenario::get("chapter1.yml").unwrap();
  println!("{:?}", std::str::from_utf8(scenario.as_ref()));

  for file in Scenario::iter() {
      println!("{}", file.as_ref());
  }

  let s = std::str::from_utf8(scenario.as_ref()).unwrap();

  let docs = YamlLoader::load_from_str(s).unwrap();

  // Multi document support, doc is a yaml::Yaml
  let doc = &docs[0];

  // Debug support
  println!("{:#?}", doc);

  // Index access for map & array
  assert_eq!(doc["Simple array"][0].as_str().unwrap(), "first");
  println!("{:?}", doc["Simple array"][3]["a"]);
  assert_eq!(doc["Simple array"][3]["a"].as_i64().unwrap(), 10);
}
