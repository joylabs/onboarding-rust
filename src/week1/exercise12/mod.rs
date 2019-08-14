pub fn palindrome(input: String) -> bool {
    let mut word = String::new();
    let mut reverse_word = String::new();

    for j in input.chars() {
        if j.is_alphanumeric() {
            match j {
                'á' => reverse_word.push('a'),
                'é' => reverse_word.push('e'),
                'í' => reverse_word.push('i'),
                'ó' => reverse_word.push('o'),
                'ú' | 'ü' => reverse_word.push('u'),
                _ => reverse_word.push(j),
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
