const VOWELS: &str = "aeiouAEIOU";
pub fn reverse_vowels(s: String) -> String {
    let mut string_vector = s
        .chars()
        .filter(|c| VOWELS.find(*c).is_some())
        .collect::<Vec<char>>();
    s.chars()
        .map(|c| {
            if VOWELS.find(c).is_some() {
                return string_vector.pop().unwrap();
            }
            c
        })
        .collect()
}