use onboarding_rust::week1::exercise14::longest_common_prefix;

#[test]
fn test_week1_exercise14_example1() {
    let input = vec!["flower", "flow", "flight"];
    let expected = "fl";

    assert_eq!(expected, longest_common_prefix(input));
}


#[test]
fn test_week1_exercise14_example2() {
    let input = vec!["dog", "racecar", "car"];
    let expected = "";

    assert_eq!(expected, longest_common_prefix(input));
}
#[test]
fn test_week1_exercise14_example3() {
    let input = vec![];
    let expected = "";

    assert_eq!(expected, longest_common_prefix(input));
}
#[test]
fn test_week1_exercise14_example4() {
    let input = vec![""];
    let expected = "";

    assert_eq!(expected, longest_common_prefix(input));
}

#[test]
fn test_week1_exercise14_example5() {
    let input = vec!["aa", "ab"];
    let expected = "a";

    assert_eq!(expected, longest_common_prefix(input));
}

#[test]
fn test_week1_exercise14_example6() {
    let input = vec!["a"];
    let expected = "a";

    assert_eq!(expected, longest_common_prefix(input));
}
#[test]
fn test_week1_exercise14_example7() {
    let input = vec!["aa"];
    let expected = "aa";

    assert_eq!(expected, longest_common_prefix(input));
}

#[test]
fn test_week1_exercise14_example8() {
    let input = vec!["c", "c"];
    let expected = "c";

    assert_eq!(expected, longest_common_prefix(input));
}
#[test]
fn test_week1_exercise14_example9() {
    let input = vec!["", ""];
    let expected = "";

    assert_eq!(expected, longest_common_prefix(input));
}

#[test]
fn test_week1_exercise14_example10() {
    let input = vec!["a", "a", "a"];
    let expected = "a";

    assert_eq!(expected, longest_common_prefix(input));
}

#[test]
fn test_week1_exercise14_example11() {
    let input = vec!["aa", "a"];
    let expected = "a";

    assert_eq!(expected, longest_common_prefix(input));
}

#[test]
fn test_week1_exercise14_example12() {
    let input = vec!["aaba", "aaa", "aa", "aa", "aa"];
    let expected = "aa";

    assert_eq!(expected, longest_common_prefix(input));
}
