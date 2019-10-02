use std::collections::HashMap;

pub fn is_match(s: String, p: String) -> bool {
    let mut mem_results: HashMap<(usize, usize), bool> = HashMap::new();
    check_pattern(0, 0, &s, &p, &mut mem_results)
}

fn check_pattern(i: usize, j: usize, s: &str, p: &str, mem_results: &mut HashMap<(usize, usize), bool>) -> bool {
    if mem_results.contains_key(&(i, j)) {
        return *(mem_results.get(&(i, j)).unwrap());
    }

    let result;

    if j == p.len() {
        result = i == s.len();
    } else {
        let first_check = i < s.len() && (p[j..=j] == s[i..=i] || &p[j..=j] == ".");
        if j + 1 < p.len() && &p[j+1..=j+1] == "*" {
            result = check_pattern(i, j + 2, s, p, mem_results) || (first_check && check_pattern(i + 1, j, s, p, mem_results));
        } else {
            result = first_check && check_pattern(i + 1, j + 1, s, p, mem_results);
        }
    }
    mem_results.insert((i, j), result);
    result
}

