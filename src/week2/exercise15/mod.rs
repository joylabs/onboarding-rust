use std::collections::HashMap;

pub fn subdomain_visits(input: Vec<String>) -> Vec<String> {
    let mut sub_domain_map: HashMap<String, i32> = HashMap::new();

    input.iter().for_each(|word| {
        get_domains(&word, &mut sub_domain_map);
    });

    sub_domain_map
        .iter()
        .map(|(k, v)| v.to_string() + " " + &k)
        .collect::<Vec<String>>()
}

fn get_domains(input: &str, map: &mut HashMap<String, i32>) {
    let mut string_pieces = input.split_whitespace();
    let visits: i32 = string_pieces.next().unwrap().parse().unwrap();
    let domain: String = string_pieces.next().unwrap().to_string();

    let input = map.entry(domain.clone()).or_insert(0);
    *input += visits;

    for (i, c) in domain.chars().enumerate() {
        if c == '.' {
            let count = map.entry((&domain[(i + 1)..]).to_string()).or_insert(0);
            *count += visits;
        }
    }

}
