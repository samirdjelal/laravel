extern crate yaml_rust;

use yaml_rust::{YamlLoader, Yaml};

pub struct Deployer {
	config: Vec<Yaml>,
}

impl Deployer {
	pub fn new() -> Self {
		Self { config: vec![] }
	}
	pub fn configure(&mut self, path: &str) {
		if !std::path::Path::new(path).exists() {
			println!("File does not exist: {}", path);
			std::process::exit(1);
		}
		let contents = std::fs::read_to_string(path).expect("Failed to read config file");
		let docs = YamlLoader::load_from_str(contents.as_str()).unwrap();
		self.config = docs.clone();
	}
	pub fn deploy(&self) {
		for doc in self.config.iter() {
			let mut yaml = doc.clone();
			
			match yaml.as_hash() {
				Some(hash) => {
					for (key, value) in hash.iter() {
						println!("{:?}: {:?}", key, value);
					}
				}
				None => {
					println!("{:?}", yaml);
				}
			}
		}
	}
}