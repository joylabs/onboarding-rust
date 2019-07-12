pub fn detect_capital(input: &str) -> bool {
    if input.chars().all(|x| x.is_uppercase()) || input.chars().all(|x| x.is_lowercase()) {
        return true;
    }
    if input.chars().filter(|x| x.is_uppercase()).count() == 1
        && input.chars().nth(0).unwrap().is_uppercase()
    {
        return true;
    }

    false
}


// OPTION ONE
// pub fn detect_capital(input: &str) -> bool {
//     let mut uppercase_counter = 0;
//     let mut lowercase_counter = 0;
//     for x in input.chars() {
//         if x.is_uppercase() {
//             uppercase_counter += 1;
//         } else {
//             lowercase_counter += 1;
//         }
//     }
//     if uppercase_counter == input.len()
//         || lowercase_counter == input.len()
//         || (input.chars().next().unwrap().is_uppercase() && uppercase_counter == 1)
//     {
//         return true;
//     }
//     false
// }