pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    if s.len() < p.len() {
        return vec![];
    }

    let mut result: Vec<i32> = vec![];
    let mut arr: Vec<i32> = vec![0; 26];

    for i in 0..p.len() {
        arr[(p.chars().nth(i).unwrap() as i32 - 'a' as i32) as usize] += 1;
    }

    for i in 0..=s.len() - p.len() {
        let temp = &s[i..(p.len() + i)];
        let mut arr1: Vec<i32> = vec![0; 26];
        for j in 0..p.len() {
            arr1[(temp.chars().nth(j).unwrap() as i32 - 'a' as i32) as usize] += 1;
        }
        if arr == arr1 {
            result.push(i as i32);
        }
    }
    result
}

pub fn find_anagrams_2(s: String, p: String) -> Vec<i32> {
    let p = sort_string(p);
    s.chars().enumerate().fold(Vec::new(), |mut acc, (i, _)| {
        if is_in_boundaries(i, &s, &p) {
            let s_sorted = build_partial_s_string(&s, i, p.len());
            let s_sorted = sort_string(s_sorted);
            if is_match(&s_sorted, &p) {
                acc.push(i as i32);
            }
        }
        acc
    })
}

fn sort_string(string: String) -> Vec<char> {
    let mut vec = string.chars().collect::<Vec<char>>();
    vec.sort();
    vec
}

fn is_in_boundaries(index: usize, s: &str, p: &[char]) -> bool {
    index < (s.len() - p.len()) + 1
}

fn build_partial_s_string(s: &str, i: usize, p_len: usize) -> String {
    s.chars()
        .enumerate()
        .filter(|(j, _c)| *j >= i && *j < (p_len + i))
        .map(|(_j, c)| c)
        .collect::<String>()
}

fn is_match(s: &[char], p: &[char]) -> bool {
    s == p
}
