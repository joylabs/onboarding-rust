
use criterion::black_box;
use criterion::Criterion;

use onboarding_rust::week1::exercise20::valid_anagram;

#[allow(dead_code)]
fn bench_valid_anagram(c: &mut Criterion) {
    c.bench_function("valid anagram", |b| {
        b.iter(|| {
            valid_anagram(
                black_box("´ra&t".to_string()),
                black_box("&ta´r".to_string()),
            )
        })
    });
}

criterion_group!(benchmarks, bench_valid_anagram,);
criterion_main!(benchmarks);
