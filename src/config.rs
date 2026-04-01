use serde::{de::DeserializeOwned, Serialize};
use std::fs;
use std::io::{self, ErrorKind};

pub fn read_json<T: DeserializeOwned>(path: &str) -> Result<T, io::Error> {
    let content = fs::read_to_string(path)?;
    serde_json::from_str(&content).map_err(|e| io::Error::new(ErrorKind::InvalidData, e))
}

pub fn write_json<T: Serialize>(path: &str, data: &T) -> Result<(), io::Error> {
    let content = serde_json::to_string_pretty(data).map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;
    fs::write(path, content)
}

pub fn read_toml<T: DeserializeOwned>(path: &str) -> Result<T, io::Error> {
    let content = fs::read_to_string(path)?;
    toml::from_str(&content).map_err(|e| io::Error::new(ErrorKind::InvalidData, e))
}

pub fn write_toml<T: Serialize>(path: &str, data: &T) -> Result<(), io::Error> {
    let content = toml::to_string_pretty(data).map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;
    fs::write(path, content)
}

pub fn read_yaml<T: DeserializeOwned>(path: &str) -> Result<T, io::Error> {
    let content = fs::read_to_string(path)?;
    serde_yaml::from_str(&content).map_err(|e| io::Error::new(ErrorKind::InvalidData, e))
}

pub fn write_yaml<T: Serialize>(path: &str, data: &T) -> Result<(), io::Error> {
    let content = serde_yaml::to_string(data).map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;
    fs::write(path, content)
}

pub fn read_auto<T: DeserializeOwned>(path: &str) -> Result<T, io::Error> {
    let ext = path.split('.').last().unwrap_or("");
    match ext {
        "json" => read_json(path),
        "toml" => read_toml(path),
        "yaml" | "yml" => read_yaml(path),
        _ => Err(io::Error::new(ErrorKind::InvalidInput, "Unsupported format")),
    }
}

pub fn write_auto<T: Serialize>(path: &str, data: &T) -> Result<(), io::Error> {
    let ext = path.split('.').last().unwrap_or("");
    match ext {
        "json" => write_json(path, data),
        "toml" => write_toml(path, data),
        "yaml" | "yml" => write_yaml(path, data),
        _ => Err(io::Error::new(ErrorKind::InvalidInput, "Unsupported format")),
    }
}
