use onboarding_rust::week2::exercise39::most_common;

#[test]
fn one_most_common() {
    let paragraph = "Bob hit a ball, the hit BALL flew far after it was hit.".to_string();
    let banned = vec!["hit".to_string()];
    let output = "ball".to_string();
    assert_eq!(output, most_common(paragraph, banned));
}

// #[test]
// fn two_most_common() {
//   let paragraph = "Bob hit a ball, the hit BALL flew far after it was hit.";
//     let banned = ["hit"];
//     let output = "ball";
//     assert_eq!(output,most_common(paragraph,banned));
// }
