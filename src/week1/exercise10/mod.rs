pub fn detect_capital_use(input: String) -> bool {
    let r: Vec<(usize, char)> = input
        .chars()
        .enumerate()
        .filter(|(_, c)| c.is_ascii_uppercase())
        .collect();

    r.is_empty() || r.len() == input.len() || (r.len() == 1 && r[0].0 == 0)
}