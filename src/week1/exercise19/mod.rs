use std::collections::HashSet;

pub fn exercise19(x: i32) -> bool {
    let mut x = x;
    let mut set = HashSet::new();

    loop {
        let mut j = 0;
        for i in x.to_string().chars() {
            j += i.to_digit(10).unwrap().pow(2);
        }
        if j == 1 {
            return true;
        } else if set.contains(&j) {
            return false;
        } else {
            set.insert(j);
            x = j as i32;
        }
    }
}