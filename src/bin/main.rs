use std::path::Path;

pub fn main() {
    let input_path = "/home/troy/Java/1brc/measurements.txt";

    let out = obrc_rs::solution(Path::new(input_path));

    check(out);
}

fn check(out: Vec<obrc_rs::ProcessedStation>) {
    let formatted = obrc_rs::format_results(&out);

    let expected_out_path = "/home/troy/Java/1brc/expected_out.txt";
    let expected = std::fs::read_to_string(expected_out_path).unwrap();
    let expected = expected.trim();
    pretty_assertions::assert_eq!(formatted, expected);
}
