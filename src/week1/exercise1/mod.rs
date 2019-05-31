pub fn hello_world(v: Vec<i64>) -> Vec<i64> {
    let mut x: Vec<i64> = Vec::new();
    for &i in v.iter() {
        if i % 2 == 0 {
            x.push(i);
        }
    }
    for &i in v.iter() {
        if i % 2 != 0 {
            x.push(i);
        }
    }
    x
}
