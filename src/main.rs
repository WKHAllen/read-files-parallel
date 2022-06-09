mod methods;
pub mod util;

use std::env;
use std::path::Path;
use std::time::{Duration, Instant};

macro_rules! separator {
    () => {
        println!();
        println!(
            "================================================================================"
        );
        println!();
    };
}

macro_rules! test_method {
    ($path:expr) => {{
        separator!();
        println!("Running serially (base case)...");
        let start = Instant::now();
        let serial_res = methods::serial::read_files($path);
        let elapsed = start.elapsed();
        println!(
            "Files: {}, Lines: {}, Chars: {}",
            serial_res.0, serial_res.1, serial_res.2
        );
        println!("Time elapsed: {} ms", elapsed.as_millis());
        (serial_res, elapsed)
    }};
    ($path:expr, $serial_res:expr, $label:literal, $function:expr) => {{
        separator!();
        println!("Running with {}...", $label);
        let start = Instant::now();
        let res = $function($path);
        let elapsed = start.elapsed();
        println!("Files: {}, Lines: {}, Chars: {}", res.0, res.1, res.2);
        println!("Time elapsed: {} ms", elapsed.as_millis());
        assert_eq!(&res, $serial_res);
        println!("Results match base case");
        elapsed
    }};
}

macro_rules! test_method_async {
    ($path:expr, $serial_res:expr, $label:literal, $function:expr) => {{
        separator!();
        println!("Running with {}...", $label);
        let start = Instant::now();
        let res = $function($path).await;
        let elapsed = start.elapsed();
        println!("Files: {}, Lines: {}, Chars: {}", res.0, res.1, res.2);
        println!("Time elapsed: {} ms", elapsed.as_millis());
        assert_eq!(&res, $serial_res);
        println!("Results match base case");
        elapsed
    }};
}

fn rank_times(times: Vec<(&str, Duration)>) {
    let mut times_ranked = times.clone();
    times_ranked.sort_by(|(_name1, duration1), (_name2, duration2)| duration1.cmp(duration2));
    let base_duration = times.iter().next().unwrap().1;

    separator!();
    println!(
        "{: <16}    {: <16}    {: <16}",
        "Method", "Duration", "Improvement"
    );
    println!("{:-<16}    {:-<16}    {:-<16}", "", "", "");

    for (label, duration) in times_ranked.iter() {
        println!(
            "{: <16}    {: >16}    {: >16}",
            label,
            format!("{} ms", duration.as_millis()),
            format!(
                "{:.3} times",
                (base_duration.as_millis() as f64) / (duration.as_millis() as f64)
            )
        );
    }

    separator!();
}

#[tokio::main]
async fn main() {
    let test_path_arg = env::args()
        .nth(1)
        .unwrap_or(env!("CARGO_MANIFEST_DIR").to_owned());
    let test_path = Path::new(&test_path_arg);

    if !test_path.exists() {
        panic!("provided path does not exist");
    } else if !test_path.is_dir() {
        panic!("provided path is not a directory");
    }

    separator!();
    println!("Path: {}", test_path.display());

    let (serial_res, serial_duration) = test_method!(&test_path);
    let threadpool_duration = test_method!(
        &test_path,
        &serial_res,
        "threadpool",
        methods::threadpool::read_files
    );
    let tokio_duration =
        test_method_async!(&test_path, &serial_res, "tokio", methods::tokio::read_files);
    let rayon_duration = test_method!(&test_path, &serial_res, "rayon", methods::rayon::read_files);

    // (serially, as a base case)
    // threadpool
    // tokio
    // rayon
    // pariter
    // monoio
    // glommio

    rank_times(vec![
        ("serial", serial_duration),
        ("threadpool", threadpool_duration),
        ("tokio", tokio_duration),
        ("rayon", rayon_duration),
    ]);
}
