use criterion::black_box;
use criterion::Criterion;

use onboarding_rust::week2::exercise2::{
    unique_morse_representations, unique_morse_representations_2,
};

#[allow(dead_code)]
fn bench_unique_morse_representations(c: &mut Criterion) {
    c.bench_function("morse representation", |b| {
        b.iter(|| {
            unique_morse_representations(black_box(vec![
                "gin".to_string(),
                "zen".to_string(),
                "gig".to_string(),
                "msg".to_string(),
            ]))
        })
    });
}

#[allow(dead_code)]
fn bench_unique_morse_representations_2(c: &mut Criterion) {
    c.bench_function("morse representation 2", |b| {
        b.iter(|| {
            unique_morse_representations_2(black_box(vec![
                "gin".to_string(),
                "zen".to_string(),
                "gig".to_string(),
                "msg".to_string(),
            ]))
        })
    });
}


criterion_group!(
    benchmarks,
    bench_unique_morse_representations,
    bench_unique_morse_representations_2,
);
criterion_main!(benchmarks);
