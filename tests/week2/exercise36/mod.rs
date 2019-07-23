use onboarding_rust::week2::exercise36::first_uniq_char;


#[test]
fn one_first_uniq_char() {
    let patter = "leetcode".to_string();
    let output = 0;
    assert_eq!(output, first_uniq_char(patter));
}

#[test]
fn two_first_uniq_char() {
    let patter = "loveleetcode".to_string();
    let output = 2;
    assert_eq!(output, first_uniq_char(patter));
}

#[test]
fn three_first_uniq_char() {
    let patter = "".to_string();
    let output = -1;
    assert_eq!(output, first_uniq_char(patter));
}