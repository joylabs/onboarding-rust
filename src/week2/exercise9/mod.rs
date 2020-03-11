use std::collections::HashMap;

pub fn is_isomorphic(s: String, t: String) -> bool {

    let mut s_hasmap = HashMap::new();
    let mut t_hasmap = HashMap::new();
    let mut s_vec = Vec::new();
    let mut t_vec = Vec::new();

    s.chars().zip(t.chars()).fold(0, |acc, x| {
        let f = s_hasmap.entry(x.0).or_insert(acc);
        s_vec.push(*f);
        let g = t_hasmap.entry(x.1).or_insert(acc);
        t_vec.push(*g);
        acc + 1
    });

    s_vec == t_vec
}

pub fn is_isomorphic_2(s: String, t: String) -> bool {

    let mut s_hasmap = HashMap::new();
    let mut t_hasmap = HashMap::new();
    let mut s_vec = Vec::new();
    let mut t_vec = Vec::new();

    s.chars().fold(0, |acc, x| {

        let f = s_hasmap.entry(x).or_insert(acc);
        s_vec.push(*f);
        acc + 1
    });

    t.chars().fold(0, |acc, x| {

        let f = t_hasmap.entry(x).or_insert(acc);
        t_vec.push(*f);
        acc + 1
    });

    s_vec == t_vec
}
