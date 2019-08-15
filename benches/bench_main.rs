#[macro_use]
extern crate criterion;

mod week1;
mod week2;

criterion_main! {
    //week1::exercise18::benchmarks,
    week1::exercise20::benchmarks,
    // week2::exercise14::benchmarks,
}
