use onboarding_rust::week2::exercise29::special_equivalent;

#[test]
fn one_special_squivalent() {
  let input =  vec!["a".to_string(),"b".to_string(),"c".to_string(),"a".to_string(),"c".to_string(),"c".to_string()];
    let output = 3;
assert_eq!(output, special_equivalent(input));
}

#[test]
fn two_special_squivalent() {
    let input = vec!["aa".to_string(),"bb".to_string(),"ab".to_string(),"ba".to_string()];
let output = 4;
  assert_eq!(output, special_equivalent(input));
}

#[test]
fn three_special_squivalent() {
    let input = vec!["abc".to_string(),"acb".to_string(),"bac".to_string(),"bca".to_string(),"cab".to_string(),"cba".to_string()];
  let output = 3;
  assert_eq!(output, special_equivalent(input));
}

#[test]
fn four_special_squivalent() {
    let input = vec!["abcd".to_string(),"cdab".to_string(),"adcb".to_string(),"cbad".to_string()];
    let output = 1;
    assert_eq!(output, special_equivalent(input));
}
