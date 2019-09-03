use onboarding_rust::week1::exercise20::is_anagram;

#[test]
fn test_week1_exercise20_anagram() {
    let input1 = String::from("anagram");
    let input2 = String::from("nagaram");
    let output = true;
    assert_eq!(output, is_anagram(input1, input2));
}

#[test]
fn test_week1_exercise20_rat() {
    let input1 = String::from("rat");
    let input2 = String::from("car");
    let output = false;
    assert_eq!(output, is_anagram(input1, input2));
}

#[test]
fn test_week1_exercise20_a() {
    let input1 = String::from("a");
    let input2 = String::from("aa");
    let output = false;
    assert_eq!(output, is_anagram(input1, input2));
}

#[test]
fn test_week1_exercise20_aacc() {
    let input1 = String::from("aacc");
    let input2 = String::from("ccac");
    let output = false;
    assert_eq!(output, is_anagram(input1, input2));
}

#[test]
fn test_week1_exercise20_ac() {
    let input1 = String::from("ac");
    let input2 = String::from("bb");
    let output = false;
    assert_eq!(output, is_anagram(input1, input2));
}