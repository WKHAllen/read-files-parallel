use super::super::util;
use std::path::Path;
use walkdir::WalkDir;

pub fn read_files(path: &Path) -> (usize, usize, usize) {
    let mut file_count = 0;
    let mut line_count = 0;
    let mut char_count = 0;

    for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
        let res = util::read_file(entry.path());
        file_count += res.0;
        line_count += res.1;
        char_count += res.2;
    }

    (file_count, line_count, char_count)
}
