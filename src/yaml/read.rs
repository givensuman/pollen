use serde_yaml::Mapping;

use std::path::Path;
use std::{fs::File, io::Read};

/// Read the provided YAML file and return a `serd_yaml::Mapping`
pub fn to_mapping(path: &Path) -> Mapping {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(error) => {
            panic!("error opening file: {:#?}", error);
        }
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => {}
        Err(error) => {
            panic!("error reading file: {:#?}", error);
        }
    };

    let yaml: Mapping = match serde_yaml::from_str(&content) {
        Ok(yaml) => yaml,
        Err(error) => {
            panic!("error parsing yaml: {:#?}", error);
        }
    };

    yaml
}
