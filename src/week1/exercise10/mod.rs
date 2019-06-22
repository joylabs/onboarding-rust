pub fn detect_capital(word: String) -> bool {
    let mut counter = 0;
    // Loop over the word
    for character in word.chars() {
        // Check if capital letter
        if (character as i32) < 97 {
            counter += 1;
        }
    }
    // Check counter and word length
    if counter == 0
        || counter == word.len()
        || counter == 1 && (word.chars().nth(0).unwrap() as i32) < 97
    {
        return true;
    }
    false
}