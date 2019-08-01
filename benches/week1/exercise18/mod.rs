
use criterion::black_box;
use criterion::Criterion;

use onboarding_rust::week1::exercise18::{single_number, single_number_2, single_number_3};

#[allow(dead_code)]
fn bench_single_number(c: &mut Criterion) {
    c.bench_function("single number", |b| {
        b.iter(|| single_number(black_box(vec![1, 4, 2, 3, 4, 1, 2])))
    });
}

#[allow(dead_code)]
fn bench_single_number_2(c: &mut Criterion) {
    c.bench_function("single number 2", |b| {
        b.iter(|| single_number_2(black_box(vec![1, 4, 2, 3, 4, 1, 2])))
    });
}

#[allow(dead_code)]
fn bench_single_number_3(c: &mut Criterion) {
    c.bench_function("single number 3", |b| {
        b.iter(|| single_number_3(black_box(vec![1, 4, 2, 3, 4, 1, 2])))
    });
}

criterion_group!(
    benchmarks,
    bench_single_number,
    bench_single_number_2,
    bench_single_number_3
);
criterion_main!(benchmarks);