pub fn common_prefix(vec: Vec<String>) -> String {
    if vec.is_empty() || vec[0].is_empty() {
        String::from("")
    } else {
        let mut input = vec;
        input.sort();
        let mut prefix = String::from("");
        let mut next_c = input.last().unwrap().chars();
        for c in input[0].chars() {
            if c == next_c.next().unwrap_or(' ') {
                prefix.push(c);
            } else {
                break;
            }
        }
        prefix
    }
}
