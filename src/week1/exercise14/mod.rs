pub fn exercise14(v: Vec<String>) -> String {
    let mut prefix = v[0].clone();
    for i in v {
        prefix = f1(prefix, i);
    }
    prefix
}

fn f1(s1: String, s2: String) -> String {
    let mut prefix: String = "".to_string();
    let mut s1_iter = s1.chars();
    let mut s2_iter = s2.chars();

    loop {
        let next_s1 = s1_iter.next();
        let next_s2 = s2_iter.next();
        if next_s1 == None || next_s2 == None {
            break;
        }
        if next_s1.unwrap() != next_s2.unwrap() {
            break;
        }
        prefix.push(next_s1.unwrap());
    }
    prefix
}

