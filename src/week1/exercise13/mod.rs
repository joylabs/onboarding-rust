pub fn exercise13(s: String) -> String {
    let mut v: Vec<char> = Vec::new();
    let mut ss: String = "".to_string();

    for i in s.chars() {
        if is_voc(i) {
            v.push(i);
        }
    }
    for i in s.chars() {
        if is_voc(i) {
            ss.push(v.pop().unwrap());
        } else {
            ss.push(i);
        }
    }
    ss
}

fn is_voc(i: char) -> bool {
    match i {
        'a' => true,
        'e' => true,
        'o' => true,
        'i' => true,
        'u' => true,
        _ => false,
    }
}