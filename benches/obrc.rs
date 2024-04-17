use serde::{Deserialize, Serialize};
use std::{
    path::Path,
    process::Command,
    time::{Duration, Instant},
};

#[derive(Serialize, Deserialize)]
struct BenchResult {
    rev: String,
    time: Duration,
}

fn main() {
    let tree_dirty = Command::new("git").arg("diff").output().unwrap();
    let tree_dirty = !tree_dirty.stdout.is_empty();

    let hash = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .unwrap();
    let hash = String::from_utf8(hash.stdout).unwrap();
    let hash = hash.trim();

    let input_path = "/home/troy/Java/1brc/measurements.txt";
    let expected_out_path = "/home/troy/Java/1brc/expected_out.txt";
    let expected = std::fs::read_to_string(expected_out_path).unwrap();
    let expected = expected.trim();

    let start = Instant::now();
    let out = obrc_rs::solution(Path::new(input_path));
    let time_taken = start.elapsed();
    println!("{}: {time_taken:?}", hash);
    let formatted = obrc_rs::format_results(&out);
    pretty_assertions::assert_eq!(formatted, expected);

    let results_path = "results.json";
    let mut og_results: Vec<BenchResult> = match std::fs::read_to_string(results_path) {
        Ok(r) => serde_json::from_str(&r).unwrap(),
        Err(_e) => {
            vec![]
        }
    };

    if tree_dirty {
        println!("ERROR: tree dirty (results will not be saved)");
        std::process::exit(1);
    } else {
        og_results.push(BenchResult {
            rev: hash.to_owned(),
            time: time_taken,
        });
        let json = serde_json::to_string(&og_results).unwrap();
        std::fs::write(results_path, json).unwrap();
    }
}
