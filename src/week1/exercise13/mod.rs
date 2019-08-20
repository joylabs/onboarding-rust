pub fn reverse_vowels(input: String) -> String {
    let mut vowels = input.chars().filter(|l| is_vowel(*l)).collect::<Vec<_>>();
    input
        .chars()
        .map(|l| {
            if is_vowel(l) {
                return vowels.pop().unwrap();
            }
            l
        })
        .collect::<String>()
}

fn is_vowel(l: char) -> bool {
    match l {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        'A' | 'E' | 'I' | 'O' | 'U' => true,
        _ => false,
    }
}
