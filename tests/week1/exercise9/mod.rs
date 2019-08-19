use onboarding_rust::week1::exercise9::reverse_string;

#[test]
fn test_week1_exercise9_hello() {
    let mut input = vec!['h', 'e', 'l', 'l', 'o'];
    let output = vec!['o', 'l', 'l', 'e', 'h'];
    reverse_string(&mut input);
    assert_eq!(output, input);
}

#[test]
fn test_week1_exercise9_hannah() {
    let mut input = vec!['H', 'a', 'n', 'n', 'a', 'h'];
    let output = vec!['h', 'a', 'n', 'n', 'a', 'H'];
    reverse_string(&mut input);
    assert_eq!(output, input);
}

#[test]
fn test_week1_exercise9_single_character() {
    let mut input = vec!['a'];
    let output = vec!['a'];
    reverse_string(&mut input);
    assert_eq!(output, input);
}

#[test]
fn test_week1_exercise9_empty_array() {
    let mut input = vec![];
    let output: Vec<char> = vec![];
    reverse_string(&mut input);
    assert_eq!(output, input);
}