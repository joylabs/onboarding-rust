use onboarding_rust::week1::exercise11::reverse_words;

#[test]
fn test_week1_exercise10_usa() {
    let input = String::from("Let's take LeetCode contest");
    let output = String::from("s'teL ekat edoCteeL tsetnoc");
    assert_eq!(output, reverse_words(input));
}