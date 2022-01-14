extern crate confy;
extern crate yaml_rust;

use std::env;
use std::fs::File;
use std::io::Read;

use serde::{Deserialize, Serialize};
use yaml_rust::{YamlEmitter, YamlLoader};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationConfig {
    pub app_name: String,
    pub port: u16,
    pub uri: String,
    pub db: String,
    pub thread_capacity: usize,
}

impl ::std::default::Default for ApplicationConfig {
    fn default() -> Self {
        Self {
            app_name: "getting-started".to_string(),
            port: 8080,
            uri: "sample-uri".to_string(),
            db: "rust".to_string(),
            thread_capacity: 10,
        }
    }
}

impl ApplicationConfig {
    pub fn load_config() -> Result<ApplicationConfig, env::VarError> {
        let key = "CONFIG_PATH";
        match env::var(key) {
            Ok(val) => match confy::load(&val) {
                Ok(val) => Ok(val),
                Err(e) => panic!("{}", e),
            },
            Err(e) => panic!("{}", e),
        }
    }

    pub fn load_yaml_config(path: String) -> ApplicationConfig {
        let mut file = File::open(path).expect("Unable to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read file");
        let docs = YamlLoader::load_from_str(&contents).expect("Error loading from string");
        let mut out_str = String::new();
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(&docs[0]).unwrap();
        serde_yaml::from_str(&out_str).expect("Error parsing")
    }
}
