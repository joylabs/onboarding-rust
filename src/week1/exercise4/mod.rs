pub fn exercise4(bb: i32, cc: i32) -> Vec<i32> {
    let mut v: Vec<i32> = vec![];
    let mut count: bool = true;

    for i in bb..=cc {
        let ss: String = i.to_string();
        let my_chars: Vec<char> = ss.chars().collect();
        for j in my_chars {
            let k = j.to_digit(10).unwrap() as i32;
            if k == 0 || i % k != 0 {
                count = false;
            }
        }
        if count {
            v.push(i);
        }
        count = true;
    }
    v
}