use std::fs;
use std::io;

pub fn read(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

pub fn read_bytes(path: &str) -> Result<Vec<u8>, io::Error> {
    fs::read(path)
}

pub fn create(path: &str) -> Result<(), io::Error> {
    fs::write(path, "")
}

pub fn create_with_content(path: &str, content: &str) -> Result<(), io::Error> {
    fs::write(path, content)
}
