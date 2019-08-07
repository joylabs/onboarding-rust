use onboarding_rust::week3::exercise2::find_content_children;

#[test]
fn test_week3_exercise2_example1() {
    let g = vec![1,2,3];
    let s = vec![1,1];
    let expected = 1;
    assert_eq!(expected, find_content_children(g, s));
}

#[test]
fn test_week3_exercise2_example2() {
    let g = vec![1,2];
    let s = vec![1,2,3];
    let expected = 2;
    assert_eq!(expected, find_content_children(g, s));
}
