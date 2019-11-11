use std::collections::HashMap;

pub fn count_of_atoms(formula: String) -> String {
    let element_count = build_map(formula.chars().collect());
    let mut formula_vec = element_count.into_iter().collect::<Vec<(String, u32)>>();
    formula_vec.sort_by(|a, b| a.0.cmp(&b.0));
    println!("{:?}", formula_vec);
    formula_vec
        .iter()
        .map(|(element, count)| {
            if *count == 1 {
                element.clone()
            } else {
                format!("{}{}", element, count)
            }
        })
        .collect::<String>()
}

fn remove_parentheses(formula: String) -> String {
    // Mg3(O2H5)2 -> Mg3H2O2
    // K4(O1N(Mg2O3)2(Rg)3)2 -> K4O2N2Mg8O12Rg6
    //[((2,17), 2) , ((5,10), 3), ((12, 15),2)] = range : Vec<((usize, usize), i32)>
    // range.inter.map(|(i,f), c)| { let x = &fomula[i..f] ->  O1N(Mg2O3)2(Rg)3 ->  O2N(Mg4O6)2(Rg)3 })
    let mut parentheses_ranges: Vec<((usize, usize), u32)>;
    for (i, c) in formula.chars().enumerate() {
        if c == '(' {
            parentheses_ranges.push(((i, 0), 0));
        }
        if c == ')' {
            for range in parentheses_ranges.iter().rev() {
                // [ (0,1), x ]
                if range.0.1 == 0 {

                }
            }
        }
    }

    let mut element = String::new();
    let mut is_upper = false;
    formula
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if c.is_ascii_digit() {
                let val = c.to_digit(10).unwrap();
            } else {
                c
            }
        })
        .collect::<String>()
}

fn build_map(vec: Vec<char>) -> HashMap<String, u32> {
    let mut element_count = HashMap::new();
    let mut element = String::new();
    let mut is_upper = false;
    vec.iter().enumerate().for_each(|(i, c)| {
        if !is_upper || c.is_ascii_lowercase() {
            if c.is_ascii_uppercase() {
                element = format!("{}{}", element, c);
                is_upper = true;
            } else if c.is_ascii_lowercase() {
                element = format!("{}{}", element, c);
            }
        }
        if c.is_ascii_digit()
            || (i + 1 < vec.len() && vec[i + 1].is_ascii_uppercase())
            || (i == vec.len() - 1)
        {
            let count = if c.is_ascii_digit() {
                // agregar mas de un digito ex. 12
                c.to_digit(10).unwrap()
            } else {
                1
            };
            *element_count.entry(element.clone()).or_insert(0) += count;
            element = "".to_string();
            is_upper = false;
        }
    });
    element_count
}
