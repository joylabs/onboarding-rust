pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
      g.sort();
      s.sort();
      let mut cookie_counter = 0;

      for child in g.into_iter() {
            if !s.is_empty() {
                  let mut cookie = s.remove(0);
                  while child > cookie && !s.is_empty() {
                        cookie = s.remove(0);
                  } 
                  if child <= cookie {
                        cookie_counter += 1;
                  }
            } else {
                  break;
            }
      }

      cookie_counter
}
