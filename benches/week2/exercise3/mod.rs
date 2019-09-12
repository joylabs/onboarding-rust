use criterion::black_box;
use criterion::Criterion;

use onboarding_rust::week2::exercise3::{
    happy_numbers, happy_numbers_2,
};

#[allow(dead_code)]
fn bench_happy_numbers(c: &mut Criterion) {
    c.bench_function("Happy Numbers", |b| {
        b.iter(|| happy_numbers(black_box(20)))
    });
}

#[allow(dead_code)]
fn bench_happy_numbers_2(c: &mut Criterion) {
    c.bench_function("Happy Numbers 2", |b| {
        b.iter(|| happy_numbers_2(black_box(20)))
    });
}

criterion_group!(
    benchmarks,
    bench_happy_numbers,
    bench_happy_numbers_2,
);
criterion_main!(benchmarks);
