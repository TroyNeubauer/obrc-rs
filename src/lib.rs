use memmap2::MmapOptions;
use std::fs::File;
use std::path::Path;

pub struct ProcessedStation {
    name: String,
    min: i8,
    avg_tmp: i32,
    avg_count: usize,
    max: i8,
}

pub fn solution(input_path: &Path) -> Vec<ProcessedStation> {
    let mut stations: Vec<ProcessedStation> = vec![];
    let file = File::open(input_path).unwrap();
    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
    let bytes: &[u8] = &mmap;

    for line in bytes.split(|b| *b == b'\n') {
        if line.is_empty() {
            continue;
        }

        let line = std::str::from_utf8(line).unwrap();
        // `City of San Marino;30.0`
        let Some((name, temp_str)) = line.split_once(';') else {
            panic!("Line missing ; seperator! {line}");
        };
        let Some((temp_int, temp_dec)) = temp_str.split_once('.') else {
            panic!("Line temp missing dot: {temp_str}");
        };
        let temp_int: i8 = temp_int.parse().unwrap();
        let temp_dec: i8 = temp_dec.parse().unwrap();
        let temp: i8 = temp_int * 10 + temp_dec;

        match stations.iter_mut().find(|i| i.name == name) {
            Some(station) => {
                if temp < station.min {
                    station.min = temp;
                }
                if temp > station.max {
                    station.max = temp;
                }

                station.avg_tmp += temp as i32;
                station.avg_count += 1;
            }
            None => {
                stations.push(ProcessedStation {
                    name: name.to_owned(),
                    min: temp,
                    avg_tmp: temp as i32,
                    avg_count: 1,
                    max: temp,
                });
            }
        }
    }

    stations.sort_unstable_by_key(|s| s.name.clone());

    stations
}

pub fn format_results(stations: &[ProcessedStation]) -> String {
    let mut out = String::new();
    out.push_str("{");
    for (i, station) in stations.iter().enumerate() {
        use std::fmt::Write;
        let avg = station.avg_tmp as f32 / 10.0 / station.avg_count as f32;
        let _ = write!(
            &mut out,
            "{}={:.1}/{:.1}/{:.1}",
            station.name, station.min, avg, station.max
        );
        if i != stations.len() - 1 {
            let _ = write!(&mut out, ", ");
        }
    }

    out.push_str("}");
    out
}

#[test]
fn validate() {
    use std::time::Instant;

    let input_path = "/home/troy/Java/1brc/measurements.txt";
    let expected_out_path = "/home/troy/Java/1brc/expected_out.txt";
    let expected = std::fs::read_to_string(expected_out_path).unwrap();
    let expected = expected.trim();

    let start = Instant::now();
    let out = solution(Path::new(input_path));
    let time_taken = start.elapsed();
    println!("Took: {time_taken:?}");
    let formatted = format_results(&out);
    pretty_assertions::assert_eq!(formatted, expected);
}
