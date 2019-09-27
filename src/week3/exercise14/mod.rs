pub fn is_match(s: String, p: String) -> bool {
    if s.is_empty() {
        if (p.len() == 2 && &p[1..] == "*") || p.is_empty() {
            return true;
        }
        return false;
    } else if p.is_empty() {
        return false;
    }

    let mut p: Vec<char> = p.chars().collect();
    let mut s: Vec<char> = s.chars().collect();

    let mut preceding_element = p.remove(0);
    let mut actual_letter = s.remove(0);

    while !s.is_empty() && !p.is_empty() {
        if actual_letter == preceding_element || preceding_element == '.' {
            actual_letter = s.remove(0);
            if p[0] != '*' {
                preceding_element = p.remove(0);
            }
        } else if p[0] == '*' {
            p.remove(0);
            if !p.is_empty() {
                preceding_element = p.remove(0);
            } else {
                preceding_element = '-';
            }

            if actual_letter == preceding_element || preceding_element == '.' {
                actual_letter = s.remove(0);
                if !p.is_empty() && p[0] != '*' {
                    preceding_element = p.remove(0);
                }
            }
        } else {
            return false;
        }

        if p.is_empty() {
            preceding_element = '-';
        }
    }

    println!("preceding_element: {}, p: {:?}", preceding_element, p);

    actual_letter == preceding_element || preceding_element == '.'
}

fn dfs_pattern() {}
