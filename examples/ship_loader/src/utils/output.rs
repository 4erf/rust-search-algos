use std::fs;
use std::path::Path;

pub fn write_string_to_file(path: &Path, string: String) -> std::io::Result<()> {
    fs::write(path, string)
}