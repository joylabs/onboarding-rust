use std::collections::HashMap;

pub fn count_of_atoms(formula: String) -> String {
    let new_formula = add_ones(formula);
    let formula_vec = to_vec(new_formula);
    let ranges = build_ranges(&formula_vec);
    let final_formula = multiply_coefficient(formula_vec, ranges);
    let element_count = build_map(final_formula);
    order_formula(element_count)
}

fn add_ones(formula: String) -> String {
    let formula_vec: Vec<char> = formula.chars().collect();

    formula
        .chars()
        .enumerate()
        .fold(String::new(), |mut acc, (i, c)| {
            if c.is_ascii_digit() || c == '(' || c == ')' {
                acc = format!("{}{}", acc, c);
            } else if (i + 1 < formula_vec.len()
                && (!formula_vec[i + 1].is_ascii_digit()
                    && !formula_vec[i + 1].is_ascii_lowercase()))
                || (i == formula_vec.len() - 1
                    && (c.is_ascii_lowercase() || c.is_ascii_uppercase()))
            {
                acc = format!("{}{}{}", acc, c, "1");
            } else {
                acc = format!("{}{}", acc, c);
            }
            acc
        })
}

fn to_vec(formula: String) -> Vec<String> {
    let mut formula_vec: Vec<String> = Vec::new();
    // iterate over formula
    let mut waiting = true;
    let mut state = "none";
    let mut handle = String::new();
    for c in formula.chars() {
        // Two states: waiting or not
        // Waiting: needs more characters, cannot push
        if !waiting {
            formula_vec.push(handle);
            waiting = true;
            handle = String::new();
        }
        // not waiting: ready to push
        // Identify symbols
        // Parentheses, no need to wait for more characters
        if c == '(' || c == ')' {
            // Push first what we have in the handle
            if handle != "" {
                formula_vec.push(handle);
            }
            handle = c.to_string();
            waiting = false;
        }
        // Numbers, need to wait for more numbers
        if c.is_digit(10) {
            // check state
            if state != "digit" && handle != "" {
                formula_vec.push(handle);
                handle = String::new();
            }
            handle = format!("{}{}", handle, c);
            waiting = true;
            state = "digit";
        }
        // Letters, need to wait
        if c.is_ascii_uppercase() {
            // check state (from another symbol)
            // println!("{} is UpperCase", c);
            if state != "upper" && handle != "" {
                formula_vec.push(handle);
                handle = String::new();
            }
            // check state (from the same symbol)
            if waiting && state == "upper" && handle != "" {
                formula_vec.push(handle.clone());
                waiting = true;
                handle = c.to_string();
            } else {
                handle = format!("{}{}", handle, c);
                waiting = true;
                state = "upper";
            }
            // println!("Current handle: {:?}", handle);
        }
        // lowercase
        if c.is_ascii_lowercase() {
            handle = format!("{}{}", handle, c);
            waiting = false;
            state = "lower";
        }
    }
    formula_vec.push(handle);
    formula_vec
}

fn build_ranges(formula_vec: &[String]) -> Vec<((usize, usize), u32)> {
    // [((2, 18), 2), ((5, 11), 2), ((13, 16), 3)]
    let mut parentheses_ranges: Vec<((usize, usize), u32)> = Vec::new();
    //let formula_vec: Vec<char> = formula.chars().collect();
    for (i, c) in formula_vec.iter().enumerate() {
        if c == "(" {
            parentheses_ranges.push(((i, 0), 0));
        }
        if c == ")" {
            for range in parentheses_ranges.iter_mut().rev() {
                if (range.0).1 == 0 {
                    (range.0).1 = i;
                    range.1 = formula_vec[i + 1].parse::<u32>().unwrap();
                    break;
                }
            }
        }
    }

    parentheses_ranges
}

fn multiply_coefficient(
    formula: Vec<String>,
    parentheses_ranges: Vec<((usize, usize), u32)>,
) -> Vec<String> {
    // K4(O1N1(Mg2O3)2(Rg1)3)2
    // [((2, 21), 2), ((7, 13), 2), ((15, 19), 3)]
    //(N1B3)33 --[((0, 5), 33)]
    let mut result: Vec<String> = vec![];
    for (i, c) in formula.iter().enumerate() {
        let mut multiplication = 0;
        // Check if "c" is a number
        if c.parse::<u32>().is_ok() {
            // compare "i" to ranges
            for r in parentheses_ranges.iter() {
                // ( (izq, der), n)
                if i >= (r.0).0 && i <= (r.0).1 {
                    // Multiply "c" by "n"
                    if multiplication == 0 {
                        multiplication += c.parse::<u32>().unwrap() * r.1;
                    } else {
                        multiplication *= r.1;
                    }
                }
            }
        }
        // Push char to Vec<&str>
        // K, (), multiplication > 0 ? multiplication.to_string : c
        if multiplication > 0 {
            result.push(multiplication.to_string());
        } else {
            result.push(c.to_string());
        }
    }

    result
        .iter()
        .enumerate()
        .filter(|(i, c)| !(c.parse::<u32>().is_ok() && (*i as i32 > 1 && result[i - 1] == ")")))
        .filter(|(_, c)| (*c != "(" && *c != ")"))
        .map(|(_, c)| c.to_string())
        .collect::<Vec<String>>()
}

fn build_map(vec: Vec<String>) -> HashMap<String, u32> {
    let mut i = 1;
    vec.iter().step_by(2).fold(HashMap::new(), |mut acc, s| {
        *acc.entry(s.to_string()).or_insert(0) += vec[i].parse::<u32>().unwrap();
        i += 2;
        acc
    })
}

fn order_formula(element_count: HashMap<String, u32>) -> String {
    let mut formula_vec = element_count.into_iter().collect::<Vec<(String, u32)>>();
    formula_vec.sort_by(|a, b| a.0.cmp(&b.0));
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
