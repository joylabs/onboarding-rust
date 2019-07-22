pub fn find_anagrams(s: &str, p: &str) -> Vec<i32> {
      let mut anagram_index: Vec<i32> = Vec::new();
      let mut alphabet_positions: Vec<i32> = vec![0; 26];
      let s_chars: Vec<char> = s.chars().collect();
      let s_len = s.len();
      let p_len = p.len();
      const OFFSET: usize = 'a' as usize;

      p.chars().for_each(|c| {
            alphabet_positions[(c as usize) - OFFSET] += 1;
      });

      for i in 0..s_len {
            alphabet_positions[(s_chars[i] as usize) - OFFSET] -= 1;

            if i >= p_len {
                  alphabet_positions[(s_chars[i - p_len] as usize) - OFFSET] += 1;
            }

            if i >= (p_len - 1) && is_anagram(&alphabet_positions) {
                  anagram_index.push((i + 1 - p_len) as i32);
            }
      }

      anagram_index
}

fn is_anagram(alphabet_positions: &[i32]) -> bool {
      [0; 26] == alphabet_positions
}
