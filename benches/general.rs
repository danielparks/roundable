//! Benchmark with [`criterion`].

#![allow(clippy::missing_docs_in_private_items, missing_docs)]

use criterion::{
    criterion_group, criterion_main, BenchmarkId, Criterion, Throughput,
};
use roundable::rot13;
use std::convert::TryInto;
use std::time::Duration;

fn benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("general");
    group
        .noise_threshold(0.10)
        .significance_level(0.01)
        .confidence_level(0.99)
        .sample_size(300)
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(10));

    for input in ["", "super secure", "super long and super secure"] {
        group.throughput(Throughput::Bytes(input.len().try_into().unwrap()));
        group.bench_with_input(
            BenchmarkId::new("rot13", input.len()),
            input,
            |b, input| b.iter(|| rot13(input)),
        );
    }

    group.finish();
}

criterion_group!(general_group, benchmarks);
criterion_main!(general_group);
