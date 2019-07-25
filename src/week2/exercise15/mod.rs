use std::collections::HashMap;

pub fn subdomain_visits(input: Vec<String>) -> Vec<String> {
      let mut sub_domain_map: HashMap<String, i32> = HashMap::new();

      input.iter().for_each(|word| {
            get_domains(&word).iter().for_each(|dom| {
                  let domain: Vec<&str> = dom.split_whitespace().collect();
                  let visits: i32 = domain[0].parse().unwrap();

                  let counter = sub_domain_map.entry(domain[1].to_string()).or_insert(0);
                  *counter += visits;
            });
      });

      sub_domain_map
            .iter()
            .map(|(k, v)| format!("{} {}", v, &k))
            .collect::<Vec<String>>()
}

fn get_domains(input: &str) -> Vec<String> {
      let mut string_pieces = input.split_whitespace();
      let visits: i32 = string_pieces.next().unwrap().parse().unwrap();
      let domain: String = string_pieces.next().unwrap().to_string();

      let mut domains = vec![input.to_string()];

      for (i, c) in domain.chars().enumerate() {
            if c == '.' {
                  domains.push(format!("{} {}", visits, &domain[(i + 1)..]));
            }
      }

      domains
}
