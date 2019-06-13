pub fn reverse_words(input: &str) -> String {
    let mut word = String::new();

    for i in input.split_whitespace() {
        for j in i.chars().rev() {
            word.push(j);
        }
        word.push(' ');
    }
    word.pop();
    word
}
