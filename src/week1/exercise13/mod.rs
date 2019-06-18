pub fn reverse_vowels_string(input: &str) -> String {
    let mut vowels = input.chars().filter(|a| is_vowel(*a)).collect::<String>();
    input
        .chars()
        .map(|c| {
            if is_vowel(c) {
                vowels.pop().unwrap()
            } else {
                c
            }
        })
        .collect::<String>()
}


fn is_vowel(letter: char) -> bool {
    match letter.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}
