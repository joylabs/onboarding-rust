pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut moves = 0;
        if word1.len() != word2.len() {
                moves += (word1.len() as i32 - word2.len() as i32).abs();
        }
        let equal_chars = word1
                .chars()
                .filter(|&c| word2.contains(c))
                .collect::<Vec<char>>();
        println!("equal_chars: {:?}", equal_chars);
        moves
}
