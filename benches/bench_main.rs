#[macro_use]
extern crate criterion;

mod week1;

criterion_main! {
    week1::exercise18::benchmarks,
}