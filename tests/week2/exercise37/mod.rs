use onboarding_rust::week2::exercise37::find_the_difference;


#[test]
fn one_word_pattern() {
    let s = "abcd";
    let t = "abcde";
    let ch = 'e';
    assert_eq!(ch, find_the_difference(s, t));
}

#[test]
fn two_word_pattern() {
    let s = "abbaba";
    let t = "ababa";
    let ch = 'b';
    assert_eq!(ch, find_the_difference(s, t));
}
