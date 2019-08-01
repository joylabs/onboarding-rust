use std::collections::BTreeMap;

pub fn count_of_atoms(formula: String) -> String {
      parse_formula(&formula)
            .iter()
            .fold("".to_string(), |mut formula, (key, &value)| {
                  formula += key;
                  if value > 1 {
                        formula += &value.to_string();
                  }

                  formula
            })
}

fn parse_formula(formula: &str) -> BTreeMap<String, i32> {
      let mut atoms_map: BTreeMap<String, i32> = BTreeMap::new();
      let mut new_formula: Vec<String> = Vec::new();
      let mut atom = String::new();
      let mut is_last_letter = false;
      let mut is_last_digit = false;

      formula.chars().for_each(|c| {
            if c == '(' || c == ')' {
                  if !atom.is_empty() {
                        new_formula.push(atom.clone());
                        atom.clear();
                  }

                  new_formula.push(c.to_string());
                  is_last_letter = false;
                  is_last_digit = false;
            } else if c.is_digit(10) {
                  if is_last_letter {
                        new_formula.push(atom.clone());
                        atom.clear();
                  }
                  atom.push(c);
                  is_last_letter = false;
                  is_last_digit = true;
            } else if c.is_lowercase() || atom.is_empty() {
                  is_last_letter = true;
                  is_last_digit = false;
                  atom.push(c);
            } else {
                  if !atom.is_empty() {
                        new_formula.push(atom.clone());
                        atom.clear();
                  }

                  atom.push(c);
                  is_last_letter = true;
                  is_last_digit = false;
            }
      });

      if !atom.is_empty() {
            new_formula.push(atom);
      }

      println!("New formula: {:?}", new_formula);

      let mut num_atom: Vec<i32> = vec![1];
      let mut is_close_parenthesis = false;
      let mut is_num = false;

      new_formula.iter_mut().rev().for_each(|s| {
            if s.parse::<i32>().is_ok() {
                  is_num = true;
                  let n = num_atom[num_atom.len() - 1];
                  num_atom.push(s.parse::<i32>().unwrap() * n);
            } else if s == ")" {
                  is_close_parenthesis = true;
                  is_num = false;
            } else if s == "(" {
                  if num_atom.len() > 1 {
                        num_atom.pop();
                  }
                  is_close_parenthesis = false;
                  is_num = false;
            } else {
                  let n = num_atom[num_atom.len() - 1];
                  let atom_count = atoms_map.entry(s.clone()).or_insert(0);
                  *atom_count += n;
                  println!("s: {}", s);
                  if num_atom.len() > 1 && is_num {
                        num_atom.pop();
                  }

                  is_close_parenthesis = false;
                  is_num = false;
            }

            println!("num_atom: {:?}", num_atom);
            println!("atoms_map: {:?}", atoms_map);
      });

      atoms_map
}
