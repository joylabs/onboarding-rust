pub fn reverse_words(s: String) -> String {
    let v: Vec<&str> = s.split(' ').collect();
    let mut output: String = String::from("");
    // Iterate over new vector
    for (i, word) in v.iter().enumerate() {
        // String (to chars) to Vector
        let mut current_word: Vec<char> = word.chars().collect();
        // Reverse current word
        current_word.reverse();
        let reversed_string: String = current_word.into_iter().collect();
        output += &reversed_string;
        // Check if last word
        if i < v.len() - 1 {
            output += " ";
        }
    }
    output
}