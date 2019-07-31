use std::collections::BTreeMap;

pub fn count_of_atoms(formula: String) -> String {
      let mut atoms_map = get_atom_names(&formula);
      count_atoms(&formula, &mut atoms_map);

      atoms_map
            .iter()
            .fold("".to_string(), |mut formula, (key, &value)| {
                  formula += key;
                  if value > 1 {
                        formula += &value.to_string();
                  }

                  formula
            })
}

fn get_atom_names(formula: &str) -> BTreeMap<String, i32> {
      let mut atoms_map: BTreeMap<String, i32> = BTreeMap::new();
      let mut atom = String::new();

      formula
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .for_each(|c| {
                  if c.is_lowercase() || atom.is_empty() {
                        atom.push(c);
                  } else {
                        atoms_map.entry(atom.clone()).or_insert(0);
                        atom.clear();
                        atom.push(c);
                  }
            });

      atoms_map.entry(atom).or_insert(0);

      println!("atoms_map: {:?}", atoms_map);

      atoms_map
}

fn count_atoms(formula: &str, atoms_map: &mut BTreeMap<String, i32>) {

}
