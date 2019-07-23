pub fn reverse_words(s: String) -> String {
        s.split(' ')
                .map(|s| s.chars().rev().collect())
                .collect::<Vec<String>>()
                .join(" ")
}
