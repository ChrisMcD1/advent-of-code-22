use std::collections::VecDeque;

use criterion::{criterion_group, criterion_main, Criterion};
use day6::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("map", |b| {
        b.iter(|| find_unique_of_size_map(include_str!("../src/real"), 14).unwrap())
    });
    c.bench_function("non-map", |b| {
        b.iter(|| find_unique_of_size(include_str!("../src/real"), 14).unwrap())
    });

    c.bench_function("add-to-buffer", |b| {
        b.iter(|| {
            let mut buffer = SmartBuffer::new();
            for _ in 0..10_000 {
                buffer.add('a')
            }
        })
    });

    c.bench_function("check-if-buffer-empty", |b| {
        b.iter(|| {
            let mut buffer = SmartBuffer::new();
            for _ in 0..10_000 {
                buffer.add('a')
            }

            for _ in 0..10_000 {
                buffer.only_has_unique();
            }
        })
    });

    c.bench_function("add-to-vec", |b| {
        b.iter(|| {
            let mut buffer = VecDeque::new();
            for _ in 0..10_000 {
                buffer.push_back('a')
            }
        })
    });

    c.bench_function("check-if-vec-empty", |b| {
        b.iter(|| {
            let mut buffer = VecDeque::new();
            for _ in 0..10_000 {
                buffer.push_back('a')
            }

            for _ in 0..10_000 {
                fully_unique(&buffer);
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
