pub fn detect_capital(word: String) -> bool {
    let input_capitals = word
        .chars()
        .filter(|c| is_capital(*c))
        .collect::<Vec<char>>();
    let capital_count = input_capitals.len();
    if capital_count == 0
        || capital_count == word.len()
        || capital_count == 1 && is_capital(word.chars().nth(0).unwrap())
    {
        return true;
    }
    false
}

fn is_capital(character: char) -> bool {
    (character as i32) < 97
}