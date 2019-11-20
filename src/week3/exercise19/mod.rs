use std::collections::HashMap;

pub fn is_match(s: String, p: String) -> bool {
    let mut mem_results: HashMap<(i32, i32), bool> = HashMap::new();
    let mut i = s.len() as i32;
    mem_results.insert((i, p.len() as i32), true);

    while i >= 0 {
        let mut previous_p_char = '-';

        p.char_indices().rev().for_each(|(j, p_char)| {
            let first_check = (i as usize) < s.len() && p_char == get_char(i, &s) || p_char == '.';

            let result = if previous_p_char == '*' {
                let r1 = check_mem(i, j + 2, &mem_results);
                let r2 = check_mem(i + 1, j, &mem_results);

                r1 || first_check && r2
            } else {
                let r1 = check_mem(i + 1, j + 1, &mem_results);

                first_check && r1
            };

            mem_results.insert((i, j as i32), result);

            previous_p_char = p_char;
        });
        i -= 1;
    }

    check_mem(0, 0, &mem_results)
}

fn check_mem(i: i32, j: usize, mem: &HashMap<(i32, i32), bool>) -> bool {
    mem.contains_key(&(i, j as i32)) && *(mem.get(&(i, j as i32)).unwrap())
}

fn get_char(i: i32, s: &str) -> char {
    s.chars().collect::<Vec<char>>()[i as usize]
}
