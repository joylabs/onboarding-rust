use onboarding_rust::week1::exercise11::reverse_words;

#[test]
fn test_week1_exercise11_example1() {

    let input = "Let's take LeetCode contest".to_string();
    let expected = "s'teL ekat edoCteeL tsetnoc".to_string();
    assert_eq!(expected, reverse_words(input));
}