use std::collections::HashMap;

pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
    let mut subdomain_count = HashMap::new();

    cpdomains.iter().for_each(|row| {
        let split = row.split_whitespace().collect::<Vec<&str>>();
        let domain_split = split[1].split('.').rev().collect::<Vec<&str>>();
        let mut tmp = String::from("");
        domain_split.iter().for_each(|d| {
            let value = split[0].parse::<i32>().unwrap();
            let key = format!("{}{}", d, tmp);
            let counter = subdomain_count.entry(key.clone()).or_insert(0);
            *counter += value;
            tmp = format!(".{}", key);
        });
    });

    subdomain_count
        .iter()
        .map(|(key, value)| format!("{} {}", value.to_string(), key.clone()))
        .collect()
}