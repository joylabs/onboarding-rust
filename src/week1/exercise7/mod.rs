pub fn exercise7(v: Vec<i32>) -> Vec<i32> {
    let v: String = v.into_iter().map(|x| x.to_string()).collect();
    let v: i32 = v.parse().unwrap();
    let vv = (v + 1).to_string();
    let mut v: Vec<i32> = vec![];
    for c in vv.chars() {
        let p = c.to_string().parse().unwrap();
        v.push(p);
    }
    v
}