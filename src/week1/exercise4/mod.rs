pub fn exercise4(b: i32, c: i32) -> Vec<i32> {
    let mut v: Vec<i32> = vec![];
    let mut count: bool = true;

    for i in b..=c {
        let s: String = i.to_string();
        let my_chars: Vec<char> = s.chars().collect();
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