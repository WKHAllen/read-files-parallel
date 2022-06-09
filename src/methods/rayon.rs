use super::super::util;
use rayon::prelude::*;
use std::path::Path;
use walkdir::WalkDir;

pub fn read_files(path: &Path) -> (usize, usize, usize) {
    WalkDir::new(&path)
        .into_iter()
        .filter_map(|e| e.ok())
        .collect::<Vec<_>>()
        .par_iter()
        .map(|entry| util::read_file(entry.path()))
        .reduce(
            || (0, 0, 0),
            |acc, current| (acc.0 + current.0, acc.1 + current.1, acc.2 + current.2),
        )
}
