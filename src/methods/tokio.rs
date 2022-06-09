use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::task;
use walkdir::WalkDir;

pub async fn read_files(path: &Path) -> (usize, usize, usize) {
    let mut handles = vec![];

    for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
        let handle = task::spawn(async move {
            match File::open(entry.path()).await {
                Ok(mut file) => {
                    let mut contents = vec![];
                    file.read_to_end(&mut contents).await.unwrap();
                    match String::from_utf8(contents.clone()) {
                        Ok(contents_str) => {
                            let files = 1;
                            let lines = contents_str.matches("\n").count();
                            let chars = contents_str.chars().count();
                            (files, lines, chars)
                        }
                        Err(_err) => (0, 0, 0),
                    }
                }
                Err(_err) => (0, 0, 0),
            }
        });

        handles.push(handle);
    }

    let mut results = vec![];

    for handle in handles {
        results.push(handle.await.unwrap());
    }

    let res = results.iter().fold((0, 0, 0), |acc, current| {
        (acc.0 + current.0, acc.1 + current.1, acc.2 + current.2)
    });

    res
}
