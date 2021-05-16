use toml::Value;

use crate::configure::{get_config_path, read_file};

pub struct BuptCanConfig {
    pub config: Option<Value>,
}

impl BuptCanConfig {
    pub fn initialize() -> Self {
        if let Some(file_data) = Self::config_from_file() {
            BuptCanConfig {
                config: Some(file_data),
            }
        } else {
            BuptCanConfig {
                config: Some(Value::Table(toml::value::Table::new())),
            }
        }
    }

    pub fn config_from_file() -> Option<Value> {
        let toml_content = match read_file(get_config_path()) {
            Ok(content) => Some(content),
            Err(_) => {
                println!("config file not found");
                None
            }
        }?;

        match toml::from_str(&toml_content) {
            Ok(parsed) => Some(parsed),
            Err(_) => {
                println!("unable to parse the config file");
                None
            }
        }
    }
}
