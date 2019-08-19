
use criterion::black_box;
use criterion::Criterion;

use onboarding_rust::week1::exercise20::{valid_anagram, valid_anagram_2};

#[allow(dead_code)]
fn bench_valid_anagram(c: &mut Criterion) {
    c.bench_function("valid anagram", |b| {
        b.iter(|| {
            valid_anagram(
                black_box("Здравствуйте".to_string()),
                black_box("Здрйвствауте".to_string()),
            )
        })
    });
}

#[allow(dead_code)]
fn bench_valid_anagram_2(c: &mut Criterion) {
    c.bench_function("valid anagram 2", |b| {
        b.iter(|| {
            valid_anagram_2(
                black_box("Здравствуйте".to_string()),
                black_box("Здрйвствауте".to_string()),
            )
        })
    });
}

criterion_group!(benchmarks, bench_valid_anagram, bench_valid_anagram_2);
criterion_main!(benchmarks);
