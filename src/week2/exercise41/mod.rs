use std::collections::HashMap;

pub fn frequency_sort(s: String) -> String {
    let elements = s.chars().fold(HashMap::new(), |mut acc, ch| {
        let c = acc.entry(ch).or_insert_with(|| "".to_string());
        c.push(ch);
        acc
    });

    let mut order_string: Vec<String> = elements.iter().map(|(_, value)| value.clone()).collect();
    order_string.sort_by(|a, b| (b.len()).cmp(&(a.len())));
    order_string.concat()
}
