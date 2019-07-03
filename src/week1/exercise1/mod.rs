
use itertools::partition;

pub fn array_by_parity(mut v: Vec<i64>) -> Vec<i64> {
    partition(&mut v, |e| *e % 2 == 0);
    v
}
