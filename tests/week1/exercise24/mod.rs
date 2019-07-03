use onboarding_rust::week1::exercise24::count_jewels;

#[test]
fn test_week1_exercise24_example1() {
    let j = String::from("aA");
    let s = String::from("aAAbbbb");
    let expected = 3;
    assert_eq!(expected, count_jewels(j, s));
}

#[test]
fn test_week1_exercise24_example2() {
    let j = String::from("z");
    let s = String::from("ZZ");
    let expected = 0;
    assert_eq!(expected, count_jewels(j, s));
}