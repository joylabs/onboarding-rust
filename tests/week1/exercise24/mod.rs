use onboarding_rust::week1::exercise24::num_jewels_in_stones;

#[test]
fn test_week1_jewels_in_stones_three() {
    let j = String::from("aA");
    let s = String::from("aAAbbbb");
    let output = 3;
    assert_eq!(output, num_jewels_in_stones(j, s));
}

#[test]
fn test_week1_jewels_in_stones_zero() {
    let j = String::from("z");
    let s = String::from("ZZ");
    let output = 0;
    assert_eq!(output, num_jewels_in_stones(j, s));
}