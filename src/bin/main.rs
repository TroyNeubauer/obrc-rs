use std::{collections::HashSet, path::Path};

pub fn main() {
    let input_path = "/home/troy/Java/1brc/measurements.txt";

    let out = obrc_rs::solution(Path::new(input_path));

    let names: Vec<_> = out
        .iter()
        .map(|o| std::str::from_utf8(&o.name).unwrap().to_owned())
        .collect();
    let names: Vec<_> = out.iter().map(|o| o.name.to_owned()).collect();
    let min = names.iter().map(|n| n.len()).min().unwrap();
    dbg!(min);

    let three_digs: HashSet<_> = names.iter().map(|n| (&n[..3]).to_owned()).collect();
    assert_eq!(three_digs.len(), names.len());

    check(out);
}

fn check(out: Vec<obrc_rs::ProcessedStation>) {
    let formatted = obrc_rs::format_results(&out);

    let expected_out_path = "/home/troy/Java/1brc/expected_out.txt";
    let expected = std::fs::read_to_string(expected_out_path).unwrap();
    let expected = expected.trim();
    pretty_assertions::assert_eq!(formatted, expected);
}
