pub fn exercise_one(v: Vec<i64>) -> Vec<i64> {
    let result: Vec<i64>;
    let w = v.clone();
    let mut even: Vec<i64> = v.into_iter().filter(|x| (x % 2) == 0).collect();
    let mut odd: Vec<i64> = w.into_iter().filter(|x| (x % 2) != 0).collect();
    even.append(&mut odd);
    result = even;
    result
}
