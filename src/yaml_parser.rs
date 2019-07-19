use yaml_rust::{YamlLoader, Yaml};

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;
use std::error::Error;

#[derive(Debug)]
pub struct YAMLParser {
    pub properties: Yaml
}

impl YAMLParser {
    pub fn new() -> YAMLParser {
        YAMLParser {
            properties: Yaml::Null
        }
    }

    pub fn seed_properties<P: AsRef<Path>>(&mut self, config_path: P)
                                           -> Result<&mut Self, Box<Error>> {
        match read_file_content(config_path) {
            Ok(data) => {
                self.properties = YAMLParser::parse_string_to_yaml(&data)?;
                Ok(self)
            },
            Err(e) => Err(Box::new(e))
        }
    }

    fn parse_string_to_yaml(str_to_parse: &str) -> Result<Yaml, Box<Error>> {
        match YamlLoader::load_from_str(str_to_parse) {
            Ok(docs) => Ok(docs[0].to_owned()),
            Err(e) => Err(Box::new(e))
        }
    }
}

fn read_file_content<P: AsRef<Path>>(path: P) -> std::io::Result<String> {
    let mut handle = File::open(path)?;
    let mut content = String::new();
    handle.read_to_string(&mut content)?;
    Ok(content)
}
