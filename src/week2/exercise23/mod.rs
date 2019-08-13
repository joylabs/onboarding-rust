pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        if board.is_empty() {
                return vec![];
        }

        words.into_iter().filter(|w| find_word(board.clone(), w.chars().collect())).collect()
}

fn find_word(mut board: Vec<Vec<char>>, word: Vec<char>) -> bool {
        for row in 0..board.len() {
                for col in 0..board[0].len() {
                        if search_for_word(&mut board, &word, 0, row as i32, col as i32) {
                                return true;
                        }
                }
        }

        false
}

fn search_for_word(mut board: &mut Vec<Vec<char>>, word: &[char], position: usize, row: i32, col: i32) -> bool {
        if row < 0 || 
        row >= board.len() as i32 || 
        col < 0 || 
        col >= board[0].len() as i32 || 
        board[row as usize][col as usize] != word[position] || 
        board[row as usize][col as usize] == '-' {
                return false;
        }

        if position + 1 == word.len() {
                return true;
        }

        let value = board[row as usize][col as usize];
        board[row as usize][col as usize] = '-';
        let new_position = position + 1;

        let found = search_for_word(&mut board, &word, new_position, row + 1, col) ||
                search_for_word(&mut board, &word, new_position, row - 1, col) ||
                search_for_word(&mut board, &word, new_position, row, col + 1) ||
                search_for_word(&mut board, &word, new_position, row, col - 1);

        board[row as usize][col as usize] = value;
        
        found
}