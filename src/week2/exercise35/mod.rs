pub fn find_anagrams(s: String, p: &str) -> Vec<i32> {
  let mut anagram_index: Vec<i32> = Vec::new();
  let s_length = s.len();
  let p_len = p.len();
  let set: Vec<char> = p.chars().collect();

  if s_length < p_len {
    return anagram_index;
  }

  for i in 0..=(s_length - p_len) {
    let j = i + p_len;
    if contains_all(&s[i..j], &set) {
      anagram_index.push(i as i32);
    }
  }

  anagram_index
}

fn contains_all(s: &str, set_p: &Vec<char>) -> bool {
  let set_s: Vec<char> = s.chars().collect();
  let mut new_vector = vec![0; 256];

  for x in 0..set_p.len() {
    new_vector[set_s[x] as usize] += 1;
    new_vector[set_p[x] as usize] -= 1;
  }
  !new_vector.into_iter().any(|x| x > 0)
}