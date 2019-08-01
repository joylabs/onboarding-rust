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
      let formula_elements = separate_formula_elements(formula);
      let mut num_atom: Vec<i32> = vec![1];
      let mut is_past_element_num = false;

      formula_elements.iter().rev().for_each(|s| {
            if s.parse::<i32>().is_ok() {
                  is_past_element_num = true;
                  let n = num_atom[num_atom.len() - 1];
                  num_atom.push(s.parse::<i32>().unwrap() * n);
            } else if s == ")" {
                  is_past_element_num = false;
            } else if s == "(" {
                  if num_atom.len() > 1 {
                        num_atom.pop();
                  }
                  is_past_element_num = false;
            } else {
                  let n = num_atom[num_atom.len() - 1];
                  let atom_count = atoms_map.entry(s.clone()).or_insert(0);
                  *atom_count += n;

                  if num_atom.len() > 1 && is_past_element_num {
                        num_atom.pop();
                  }
                  is_past_element_num = false;
            }
      });

      atoms_map
}

fn separate_formula_elements(formula: &str) -> Vec<String> {
      let mut formula_elements: Vec<String> = Vec::new();
      let mut element = String::new();
      let mut is_last_letter = false;

      formula.chars().for_each(|c| {
            if c == '(' || c == ')' {
                  if !element.is_empty() {
                        formula_elements.push(element.clone());
                        element.clear();
                  }
                  formula_elements.push(c.to_string());
                  is_last_letter = false;
            } else if c.is_digit(10) {
                  if is_last_letter {
                        formula_elements.push(element.clone());
                        element.clear();
                  }
                  element.push(c);
                  is_last_letter = false;
            } else if c.is_lowercase() || element.is_empty() {
                  is_last_letter = true;
                  element.push(c);
            } else {
                  if !element.is_empty() {
                        formula_elements.push(element.clone());
                        element.clear();
                  }
                  element.push(c);
                  is_last_letter = true;
            }
      });

      if !element.is_empty() {
            formula_elements.push(element);
      }

      formula_elements
}
