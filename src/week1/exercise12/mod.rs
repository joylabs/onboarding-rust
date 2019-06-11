pub fn palindrome(input: String) -> bool {
    let mut word = String::new();
    let mut reverse_word = String::new();


    for i in input.split_whitespace() {
        for j in i.chars() {
            if j.is_alphanumeric() {
                reverse_word.push(j);
            }
        }
    }

    for j in reverse_word.chars().rev() {
        word.push(j);
    }

    word = word.to_lowercase();
    reverse_word = reverse_word.to_lowercase();

    word == reverse_word
}