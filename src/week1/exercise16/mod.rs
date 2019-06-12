pub fn exercise16(x: i32, y: i32) -> i32 {
    if x > y {
        return iterate(format!("{:b}", x), format!("{:b}", y));
    } else {
        return iterate(format!("{:b}", y), format!("{:b}", x));
    }
    0
}

fn iterate(s1: String, mut s2: String) -> i32 {
    let mut sum = 0;
    for i in s1.chars().rev() {
        let next_s2 = s2.pop();
        match i {
            i if i != '0' && next_s2 == None => sum += 1,
            i if next_s2 != None && i != next_s2.unwrap() => sum += 1,
            _ => (),
        }
    }
    sum
}