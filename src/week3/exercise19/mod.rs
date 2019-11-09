use std::collections::HashMap;

pub fn is_match(s: String, p: String) -> bool {
    let mut mem_results: HashMap<(i32, i32), bool> = HashMap::new();
    let mut i = s.len() as i32;
    mem_results.insert((s.len() as i32, p.len() as i32), true);

    while i >= 0 {
        let mut j = if p.is_empty() {
            -1
        } else {
            (p.len() - 1) as i32
        };

        while j >= 0 {
            let first_check = (i as usize) < s.len()
                && p[j as usize..=j as usize] == s[i as usize..=i as usize]
                || &p[j as usize..=j as usize] == ".";

            if ((j + 1) as usize) < p.len() && &p[(j + 1) as usize..=(j + 1) as usize] == "*" {
                let r1 = if mem_results.contains_key(&(i, j + 2)) {
                    *(mem_results.get(&(i, j + 2)).unwrap())
                } else {
                    false
                };

                let r2 = if mem_results.contains_key(&(i + 1, j)) {
                    *(mem_results.get(&(i + 1, j)).unwrap())
                } else {
                    false
                };

                let result = r1 || first_check && r2;
                mem_results.insert((i, j), result);
            } else {
                let r1 = if mem_results.contains_key(&(i + 1, j + 1)) {
                    *(mem_results.get(&(i + 1, j + 1)).unwrap())
                } else {
                    false
                };

                let result = first_check && r1;
                mem_results.insert((i, j), result);
            }
            j -= 1;
        }
        i -= 1;
    }

    if mem_results.contains_key(&(0, 0)) {
        *(mem_results.get(&(0, 0)).unwrap())
    } else {
        false
    }
}
