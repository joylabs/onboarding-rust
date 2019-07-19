fn is_caps(c: char) -> bool {
        let d = c as i32 - 0 as i32;
        const A: i32 = 'A' as i32 - 0 as i32;
        const Z: i32 = 'Z' as i32 - 0 as i32;

        d >= A && d <= Z
}

fn all_caps(s: String) -> bool {
        s.chars().all(is_caps)
}

fn is_lower(c: char) -> bool {
        let d = c as i32 - 0 as i32;
        const A: i32 = 'a' as i32 - 0 as i32;
        const Z: i32 = 'z' as i32 - 0 as i32;

        d >= A && d <= Z
}

fn all_lower(s: String) -> bool {
        s.chars().all(is_lower)
}

fn first_caps(s: String) -> bool {
        all_caps(s[..0].to_string()) && all_lower(s[1..].to_string())
}

pub fn detect_capital_use(word: String) -> bool {
        all_lower(word.clone()) || all_caps(word.clone()) || first_caps(word.clone())
}
