use std::collections::HashMap;

pub fn first_uniq_char(input: String) -> i32 {
    let mut mapa = HashMap::new();

    input.chars().enumerate().for_each(|(i, ch)| {
        let count = mapa.entry(ch).or_insert((i, 0));
        count.1 += 1;
    });

    let output: Vec<(usize, i32)> = mapa
        .iter()
        .filter(|(_, (_, v))| *v == 1)
        .map(|(_, (p, v))| (*p, *v))
        .collect();

    if !output.is_empty() {
        output.iter().min().map(|(p, _)| *p).unwrap() as i32
    } else {
        -1 as i32
    }
}
