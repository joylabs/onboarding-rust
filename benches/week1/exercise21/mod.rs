
use criterion::black_box;
use criterion::Criterion;

use onboarding_rust::week1::exercise21::{find_circle_num, find_circle_num_2};

#[allow(dead_code)]
fn bench_find_circle_num(c: &mut Criterion) {
    c.bench_function("find friend circles", |b| {
        b.iter(|| {
            find_circle_num(black_box(vec![
                vec![1, 0, 0, 1],
                vec![0, 1, 1, 0],
                vec![0, 1, 1, 1],
                vec![1, 0, 1, 1],
            ]))
        })
    });
}

#[allow(dead_code)]
fn bench_find_circle_num_2(c: &mut Criterion) {
    c.bench_function("find friend circles 2", |b| {
        b.iter(|| {
            find_circle_num_2(black_box(vec![
                vec![1, 0, 0, 1],
                vec![0, 1, 1, 0],
                vec![0, 1, 1, 1],
                vec![1, 0, 1, 1],
            ]))
        })
    });
}

criterion_group!(benchmarks, bench_find_circle_num, bench_find_circle_num_2);
criterion_main!(benchmarks);
