use onboarding_rust::week1::exercise9::reverse_string;

#[test]
fn test_week1_exercise9_example1() {
    let input = vec!['h', 'e', 'l', 'l', 'o'];
    let expected = vec!['o', 'l', 'l', 'e', 'h'];
    assert_eq!(expected, reverse_string(input));
}

#[test]
fn test_week1_exercise9_example2() {
    let input = vec!['H', 'a', 'n', 'n', 'a', 'h'];
    let expected = vec!['h', 'a', 'n', 'n', 'a', 'H'];
    assert_eq!(expected, reverse_string(input));
}