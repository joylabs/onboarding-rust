pub fn is_match(s: String, p: String) -> bool {
    if s.is_empty() {
        let p = &(clear_pattern(&p));
        println!("p: {:?}", p);
        if p.is_empty() {
            return true;
        }
        return false;
    } else if p.is_empty() || (p.len() > s.len() && !p.contains('*')) {
        return false;
    }

    dfs_pattern_check(&(s.chars().collect::<Vec<char>>()), &(p.chars().collect::<Vec<char>>()), false)
}

fn dfs_pattern_check(s: &[char], p: &[char], zero_or_more: bool) -> bool {
    println!("s: {:?}, p: {:?}", s, p);
    if p.is_empty() && !s.is_empty() {
        return false;
    }

    if s.len() == 1 {
        if p.len() > 1 {
            if p[p.len() - 1] == '*' {
                println!("*");
                return dfs_pattern_check(s, &p[0..(p.len() - 1)], true);
            } else if p.len() > 2 && p[1] == '*' {
                println!("p.len() > 2 && p[1] == '*', p element = {:?}", &p[2..(p.len())]);
                if (s[0] != p[p.len() - 1] && p[p.len() - 1] != '.') && !zero_or_more {
                    return false;
                } else if (s[0] == p[p.len() - 1] || p[p.len() - 1] == '.') && s[0] != p[0] {
                    return dfs_pattern_check(s, &p[2..(p.len())], false);
                } else {
                    return dfs_pattern_check(s, &p[0..(p.len() - 1)], false);
                }
            } else if zero_or_more {
                println!("zero_or_more");
                return dfs_pattern_check(s, &p[0..(p.len() - 1)], false);
            } else {
                println!("else");
                return false;
            }
        } else if p.len() == 1 {
            return s[0] == p[0] || p[0] == '.';
        } else {
            return false;
        }
    }

    let s_element = s[s.len() - 1];
    let p_element = p[p.len() - 1];

    if p_element == s_element {
        if zero_or_more {
            dfs_pattern_check(&s[0..(s.len() - 1)], p, true)
        } else {
            dfs_pattern_check(&s[0..(s.len() - 1)], &p[0..(p.len() - 1)], false)
        }
    } else if p_element == '.' {
        if zero_or_more {
            if p.len() >= 2 && s_element == p[p.len() - 2]{
                dfs_pattern_check(&s[0..(s.len() - 1)], &p[0..(p.len() - 2)], false)
            } else {
                dfs_pattern_check(&s[0..(s.len() - 1)], p, true)
            }
        } else {
            dfs_pattern_check(&s[0..(s.len() - 1)], &p[0..(p.len() - 1)], false)
        }
    } else if p_element == '*' {
        dfs_pattern_check(s, &p[0..(p.len() - 1)], true)
    } else {
        if zero_or_more {
            dfs_pattern_check(s, &p[0..(p.len() - 1)], false)
        } else {
            false
        }
    }
}

fn clear_pattern(p: &str) -> String {
    p.chars().rev().fold((Vec::new(), false), |(mut s, mut asterisk), c| {
        if c == '*' {
            asterisk = true;
        } else if !asterisk {
            s.push(c);
            asterisk = false;
        } else {
            asterisk = false;
        }
        (s, asterisk)
    }).0.iter().rev().collect::<String>()
}