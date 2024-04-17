use std::path::Path;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench");
    // Limit sample size since this takes a while
    group.sample_size(10);

    group.bench_function("one-billion-lines", |b| {
        b.iter(|| {
            let input_path = "/home/troy/Java/1brc/measurements.txt";

            let out = obrc_rs::solution(Path::new(input_path));
            let formatted = obrc_rs::format_results(&out);
            let _ = black_box(formatted);
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
