fn exercise16(x:i32, y:i32) {
    let z = format!("{:b}", x);
    let v = format!("{:b}", y);
    if v.len() > z.len() {
        iterate(v, z);
    } else {
        iterate(z, v);
    }
}

fn iterate(s1: String, mut s2: String) {
    let mut sum = 0;
    for i in s1.chars().rev() {
        let next_s2 = s2.pop();
        if next_s2 == None {
            sum += 1;
        } else {
            match i {
                next_s2 => (),
                i if i != next_s2.unwrap() => sum += 1,
                _ => (),
            }
        }
    }
}