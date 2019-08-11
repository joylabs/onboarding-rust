use regex::Regex;
use std::collections::HashMap;

pub fn count_of_atoms(formula: String) -> String {
    let re = Regex::new(r"(?P<elem>[A-Z][a-z]*)(?P<m1>\d*)|(?P<open>\()|(?P<close>\))(?P<m2>\d*)")
        .unwrap();

    let mut result: Vec<HashMap<String, u32>> = vec![HashMap::new()];
    for caps in re.captures_iter(&formula) {
        if let Some(elem) = caps.name("elem") {
            let molecula = caps
                .name("m1")
                .map(|m| m.as_str())
                .unwrap_or("1")
                .parse::<u32>()
                .unwrap_or(1);
            result
                .last_mut()
                .unwrap()
                .entry(elem.as_str().to_owned())
                .and_modify(|e| *e += molecula)
                .or_insert(molecula);

        } else if caps.name("open").is_some() {
            result.push(HashMap::new());
        } else if caps.name("close").is_some() {
            let last = result.pop().unwrap();
            last.into_iter().for_each(|(elem, m)| {
                let m2 = caps
                    .name("m2")
                    .map(|m| m.as_str())
                    .unwrap_or("1")
                    .parse::<u32>()
                    .unwrap();
                result
                    .last_mut()
                    .unwrap()
                    .entry(elem)
                    .and_modify(|e| *e += m * m2)
                    .or_insert(m * m2);
            });
        }
    }
    parse_to_stirng(&result[0])
}

fn parse_to_stirng(formula: &HashMap<String, u32>) -> String {
    let mut total_atoms: Vec<String> = formula
        .iter()
        .map(|(k, v)| {
            if *v == 1 {
                k.clone()
            } else {
                k.clone() + &v.to_string()
            }
        })
        .collect();
    total_atoms.sort();
    total_atoms.concat()
}