pub fn is_subsequence(s: String, t: String) -> bool {
      let vec_s: Vec<char> = s.chars().collect();
      let filtered_t: Vec<char> = t.chars().filter(|c| vec_s.contains(c)).collect();

      let mut i: usize = 0;
      let mut j: usize = 0;
      while j < vec_s.len() && i < filtered_t.len() {
            if vec_s[j] == filtered_t[i] {
                  j += 1;
            }
            i += 1;
      }

      j == vec_s.len()
}
