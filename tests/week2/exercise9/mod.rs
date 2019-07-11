use onboarding_rust::week2::exercise9::is_isomorphic;

#[test]
fn test_week2_exercise9_example1() {
    let s = String::from("egg");
    let t = String::from("add");
    let expected = true;
    assert_eq!(expected, is_isomorphic(s, t));
}

#[test]
fn test_week2_exercise9_example2() {
    let s = String::from("foo");
    let t = String::from("bar");
    let expected = false;
    assert_eq!(expected, is_isomorphic(s, t));
}

#[test]
fn test_week2_exercise9_example3() {
    let s = String::from("paper");
    let t = String::from("title");
    let expected = true;
    assert_eq!(expected, is_isomorphic(s, t));
}

#[test]
fn test_week2_exercise9_example4() {
    let s = String::from("aba");
    let t = String::from("baa");
    let expected = false;
    assert_eq!(expected, is_isomorphic(s, t));
}

#[test]
fn test_week2_exercise9_example5() {
    let s = String::from("aab");
    let t = String::from("aaa");
    let expected = false;
    assert_eq!(expected, is_isomorphic(s, t));
}

#[test]
fn test_week2_exercise9_example6() {
    let s = String::from("aaaaabbbbbcccccdddddeeeee");
    let t = String::from("pppppqqqqqrrrrrsssssttttt");
    let expected = true;
    assert_eq!(expected, is_isomorphic(s, t));
}

#[test]
fn test_week2_exercise9_example7() {
    let s = String::from("abcdefghijklmnopqrstuvwxyz");
    let t = String::from("nopqrstuvwxyzabcdefghijklm");
    let expected = true;
    assert_eq!(expected, is_isomorphic(s, t));
}

#[test]
fn test_week2_exercise9_example8() {
    let s = String::from("abba");
    let t = String::from("abab");
    let expected = false;
    assert_eq!(expected, is_isomorphic(s, t));
}