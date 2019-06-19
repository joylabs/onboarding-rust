pub fn exercise18(v: Vec<i32>) -> i32 {

    for i in &v {
        let mut is = 0;
        for j in &v {
            if i == j {
                is += 1;
            }
        }
        if is < 2 {
            return *i;
        }
    }
    0
}
