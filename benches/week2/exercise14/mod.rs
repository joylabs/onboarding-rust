use criterion::black_box;
use criterion::Criterion;

use onboarding_rust::week2::exercise14::{
    first_unique_char, first_unique_char_2, first_unique_char_3,
};

#[allow(dead_code)]
fn bench_single_number(c: &mut Criterion) {
    c.bench_function("first unique char", |b| {
        b.iter(|| first_unique_char(black_box("loveleetcode".to_string())))
    });
}

#[allow(dead_code)]
fn bench_single_number_2(c: &mut Criterion) {
    c.bench_function("first unique char 2", |b| {
        b.iter(|| first_unique_char_2(black_box("loveleetcode".to_string())))
    });
}

#[allow(dead_code)]
fn bench_single_number_3(c: &mut Criterion) {
    c.bench_function("first unique char 3", |b| {
        b.iter(|| first_unique_char_3(black_box("loveleetcode".to_string())))
    });
}

criterion_group!(
    benchmarks,
    bench_single_number,
    bench_single_number_2,
    bench_single_number_3
);
criterion_main!(benchmarks);
