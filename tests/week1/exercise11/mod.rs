use onboarding_rust::week1::exercise11::reverse_words;

#[test]
fn test_week1_exercise11_leet_code() {
    let input: String = String::from("Let's take LeetCode contest");
    let output: String = String::from("s'teL ekat edoCteeL tsetnoc");
    assert_eq!(output, reverse_words(input));
}

#[test]
fn test_week1_exercise11_hello_world() {
    let input: String = String::from("Hello, World!");
    let output: String = String::from(",olleH !dlroW");
    assert_eq!(output, reverse_words(input));
}

#[test]
fn test_week1_exercise11_no_spaces() {
    let input: String = String::from("NoSpaces");
    let output: String = String::from("secapSoN");
    assert_eq!(output, reverse_words(input));
}