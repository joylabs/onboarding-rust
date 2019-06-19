
use std::collections::HashMap;

pub fn exercise20(v: Vec<i32>, x: i32) -> Vec<i32> {
  let key: HashMap<i32, i32> = v
    .iter()
    .enumerate()
    .map(|(i, item)| (*item, i as i32))
    .collect();

  v.iter()
    .enumerate()
    .find(|(i, item)| key.get(&(x - *item)).map_or(false, |j| *i as i32 != *j))
    .map(|(i, item)| vec![i as i32, key[&(x - item)]])
    .unwrap()
}

