use onboarding_rust::week1::exercise11::reverse_words;

#[test]
fn test_week1_exercise11_example1() {

    let input = "Let's take LeetCode contest";
    let expected = "s'teL ekat edoCteeL tsetnoc";
    assert_eq!(expected, reverse_words(input));
}

#[test]
fn test_week1_exercise11_example2() {

    let input = "hello my friend";
    let expected = "olleh ym dneirf";
    assert_eq!(expected, reverse_words(input));
}
