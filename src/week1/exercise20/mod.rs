pub fn valid_anagram_2(input_1: String, input_2: String) -> bool {
    !input_1.chars().any(|c| !input_2.contains(c))
}

pub fn valid_anagram(input_1: String, input_2: String) -> bool {
    let mut vec_input_1: Vec<char> = input_1.chars().collect();
    let mut vec_input_2: Vec<char> = input_2.chars().collect();

    vec_input_1.sort();
    vec_input_2.sort();
    vec_input_1 == vec_input_2
}
