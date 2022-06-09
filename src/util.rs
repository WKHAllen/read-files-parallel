use std::fs;
use std::path::Path;

pub fn read_file(path: &Path) -> (usize, usize, usize) {
    match fs::read_to_string(path) {
        Ok(contents) => (1, contents.matches("\n").count(), contents.chars().count()),
        Err(_err) => (0, 0, 0),
    }
}
