pub fn title_to_number(s: String) -> i32 {
        s.chars()
                .fold(0, |r, ch| r * 26 + ch as i32 - 'A' as i32 + 1)
}
