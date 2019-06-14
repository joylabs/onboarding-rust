use onboarding_rust::week1::exercise11::exercise11;

#[test]
fn test_1_week1_exercise11() {
    let a = String::from("Let's take LeetCode contest");
    let b = String::from("s'teL ekat edoCteeL tsetnoc");
    assert_eq!(b, exercise11(a));
}
