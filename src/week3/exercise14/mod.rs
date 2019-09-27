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

    let preceding_element = p.remove(0);
    let actual_letter = s.remove(0);

    dfs_pattern(actual_letter, preceding_element, &s, &p)
}

fn dfs_pattern(
    mut actual_letter: char,
    mut preceding_element: char,
    s: &[char],
    p: &[char],
) -> bool {
    println!(
        "preceding_element: {}, actual_letter: {}, s: {:?}, p: {:?}",
        preceding_element, actual_letter, s, p
    );

    if !s.is_empty() && p.is_empty() {
        println!("!s.is_empty() && p.is_empty()");
        return false;
    }

    if actual_letter == preceding_element || preceding_element == '.' {
        if s.is_empty() {
            println!(
                "s.is_empty() && (actual_letter == preceding_element || preceding_element == '.')"
            );
            let p = process_pattern(p);
            return p.is_empty() || (p.len() == 1 && p[0] == actual_letter);
        }
        actual_letter = s[0];
        if !p.is_empty() && p[0] != '*' {
            preceding_element = p[0];
            dfs_pattern(actual_letter, preceding_element, &s[1..], &p[1..])
        } else {
            dfs_pattern(actual_letter, preceding_element, &s[1..], p)
        }
    } else if !p.is_empty() && p[0] == '*' {
        if p.len() >= 2 {
            preceding_element = p[1];
            dfs_pattern(actual_letter, preceding_element, s, &p[2..])
        } else {
            false
        }
    } else {
        println!("else");
        false
    }
}

fn process_pattern(p: &[char]) -> Vec<char> {
    let mut processed_patter = Vec::new();
    let mut asterisk = false;
    for c in p.iter().rev() {
        if *c == '*' {
            asterisk = true;
        } else if !asterisk {
            processed_patter.push(*c);
            asterisk = false;
        }
    }
    processed_patter.reverse();
    processed_patter
}
