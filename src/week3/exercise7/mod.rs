pub fn peak_index_in_mountain_array(a: Vec<i32>) -> i32 {
    a.iter()
        .enumerate()
        .skip(1)
        .find(|(i, item)| Some(i).map_or(false, |j| a[j - 1] < **item && a[j + 1] < **item))
        .map(|(i, _)| i as i32)
        .unwrap()
}

