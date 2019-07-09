use std::collections::HashSet;

pub fn is_isomorphic(s: String, t: String) -> bool {
   let s2: Vec<char> = s.chars().collect::<HashSet<char>>().into_iter().collect();
   let t2: Vec<char> = t.chars().collect::<HashSet<char>>().into_iter().collect();
   
   s2.len() == t2.len() 
}
