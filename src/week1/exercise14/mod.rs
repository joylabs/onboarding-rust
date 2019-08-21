pub fn longest_common_prefix(input: Vec<String>) -> String {
    if input.is_empty() {
        return String::from("");
    }
    let prefix = input[0].clone();

    input.into_iter().skip(1).fold(prefix, compare_strings)
}

fn compare_strings(str1: String, str2: String) -> String {
    str1.chars()
        .zip(str2.chars())
        .enumerate()
        .fold(String::new(), |acc, (i, (c1, c2))| {
            if c1 == c2 && acc.len() == i {
                format!("{}{}", acc, c1.to_string())
            } else {
                acc
            }
        })
}