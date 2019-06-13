pub fn exercise20(v: Vec<i32>, x:i32) -> Vec<i32> {
    let vv = v.len();
    for i in 0..vv {
        for j in 0..vv {
            if x == v[j] + v[i] {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![0, 0]
}