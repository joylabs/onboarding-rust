pub fn is_palindrome(s: String) -> bool {

        let s = s
                .chars()
                .map(|c| char::to_ascii_lowercase(&c))
                .filter(|c| char::is_ascii_alphanumeric(c))
                .collect::<String>();

        s == s.chars().rev().collect::<String>()
}
