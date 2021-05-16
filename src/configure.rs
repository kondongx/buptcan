use std::{ffi::OsString, fs::File, path::Path};
use std::{
    io::{Read, Result, Write},
    process,
};

use toml::{
    value::{Map, Table},
    Value,
};

use crate::config::BuptCanConfig;

pub fn get_configuration() -> Value {
    BuptCanConfig::initialize()
        .config
        .expect("Failed to load buptcan config")
}

pub fn update_configuration(name: &str, value: &str) {
    let keys: Vec<&str> = name.split(".").collect();
    if keys.len() != 2 {
        println!("Please pass in a config key with a '.'");
        process::exit(1);
    }

    if let Some(table) = get_configuration().as_table_mut() {
        if !table.contains_key(keys[0]) {
            table.insert(keys[0].to_string(), Value::Table(Map::new()));
        }

        if let Some(values) = table.get(keys[0]).unwrap().as_table() {
            let mut updated_values = values.clone();

            updated_values.insert(keys[1].to_string(), Value::String(value.to_string()));
            table.insert(keys[0].to_string(), Value::Table(updated_values));
        }

        write_configuration(table)
    }
}

pub fn write_configuration(table: &mut Table) {
    let config_path = get_config_path();

    let config_str = toml::to_string_pretty(table).expect("Failed to serialize config to string");

    File::create(config_path)
        .and_then(|mut file| file.write_all(config_str.as_ref()))
        .expect("Error writing config");
}

pub fn edit_configuration() {}

pub fn get_config_path() -> OsString {
    // Not support env yet!
    dirs_next::home_dir()
        .expect("couldn't not find home directory")
        .join(".config")
        .join("buptcan.toml")
        .into()
}

pub fn read_file<P: AsRef<Path>>(file_name: P) -> Result<String> {
    let mut file = File::open(file_name)?;
    let mut data = String::new();

    file.read_to_string(&mut data)?;
    Ok(data)
}

#[cfg(test)]
mod tests {}
