use onboarding_rust::week1::exercise46::find_content_children;

#[test]
fn test_week1_exercise46_one_assigned_cookie() {
    let children = vec![1,2,3];
    let cookies = vec![1,1];
    let output = 1;
    assert_eq!(output, find_content_children(children, cookies));
}

#[test]
fn test_week1_exercise46_two_assigned_cookie() {
    let children = vec![1,2];
    let cookies = vec![1,2,3];
    let output = 2;
    assert_eq!(output, find_content_children(children, cookies));
}