pub fn exercise7(mut v: Vec<i32>) -> Vec<i32> {
    let size = v.len() - 1;
    v[size] = v.get(size).unwrap() + 1;
    v
}