pub fn order_numbers(v: Vec<i64>) -> Vec<i64> {
    let x = v.clone();
    let mut even: Vec<i64> = v.into_iter().filter(|x| (x % 2) == 0).collect();
    let mut odd: Vec<i64> = x.into_iter().filter(|x| (x % 2) != 0).collect();
    even.append(&mut odd);
    even
}
