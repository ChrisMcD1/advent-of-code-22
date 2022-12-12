use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day4::{part_1_filter, part_1_map};

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../src/input.txt");
    c.bench_function("part_1_filter", |b| b.iter(|| part_1_filter(input)));
    c.bench_function("part_1_map", |b| b.iter(|| part_1_map(input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
