
pub fn reverse_vowels(s: String) -> String {

        const AEIOU: &str = "aeiouAEIOU";

        let mut vowels = s
                .chars()
                .filter(|c| AEIOU.contains(*c))
                .collect::<Vec<char>>();

        s.chars()
                .map(|c| {

                        if AEIOU.contains(c) {
                                vowels.pop().unwrap()
                        } else {
                                c
                        }
                })
                .collect::<String>()
}