pub fn detect_capital(input: &str) -> bool {
    if input.chars().all(|x| x.is_uppercase()) || input.chars().all(|x| x.is_lowercase()) {
        return true;
    }
    if input.chars().filter(|x| x.is_uppercase()).count() == 1
        && input.chars().nth(0).unwrap().is_uppercase()
    {
        return true;
    }

    false
}
