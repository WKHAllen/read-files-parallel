use super::super::util;
use std::path::Path;
use std::sync::mpsc;
use threadpool::ThreadPool;
use walkdir::WalkDir;

pub fn read_files(path: &Path) -> (usize, usize, usize) {
    let max_threads = 100;
    let pool = ThreadPool::new(max_threads);

    let (tx, rx) = mpsc::channel();

    let mut num_jobs = 0;

    for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
        let tx_clone = tx.clone();

        pool.execute(move || {
            let res = util::read_file(entry.path());
            tx_clone.send(res).unwrap();
        });

        num_jobs += 1;
    }

    let res = rx.iter().take(num_jobs).fold((0, 0, 0), |acc, current| {
        (acc.0 + current.0, acc.1 + current.1, acc.2 + current.2)
    });

    res
}
