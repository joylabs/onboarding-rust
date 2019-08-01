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
      let mut num_atom = String::new();

      formula.chars().for_each(|c| {
            if c == '(' || c == ')' {
                  if !atom.is_empty() {
                        new_formula.push(atom.clone());
                        atom.clear();
                  }
                  if !num_atom.is_empty() {
                        new_formula.push(num_atom.clone());
                        num_atom.clear();
                  }
                  new_formula.push(c.to_string());
            } else if c.is_digit(10) {
                  num_atom.push(c);
            } else if c.is_lowercase() || atom.is_empty() {
                  atom.push(c);
            } else {
                  if !atom.is_empty() {
                        new_formula.push(atom.clone());
                        atom.clear();
                  }
                  if !num_atom.is_empty() {
                        new_formula.push(num_atom.clone());
                        num_atom.clear();
                  }
                  atom.push(c);
            }
      });

      if !atom.is_empty() {
            new_formula.push(atom);
      }
      if !num_atom.is_empty() {
            new_formula.push(num_atom);
      }

      // println!("New formula: {:?}", new_formula);

      let mut num_atom: Vec<i32> = vec![1];
      let mut is_close_parenthesis = false;
      let mut is_num = false;

      let mut atoms: Vec<String> = Vec::new();

      new_formula.iter_mut().rev().for_each(|s| {
            // println!("num_atom: {:?}", num_atom);
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
                  for _ in 0..n {
                        atoms.push(s.clone());
                  }
                  if num_atom.len() > 1 && is_num {
                        num_atom.pop();
                  }
                  is_close_parenthesis = false;
                  is_num = false;
            }
      });

      // println!("atoms: {:?}", atoms);

      atoms.iter().for_each(|s| {
            let atom_count = atoms_map.entry(s.clone()).or_insert(0);
            *atom_count += 1;
      });

      atoms_map
}
