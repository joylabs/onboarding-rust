use onboarding_rust::week1::exercise7::exercise7;

#[test]
fn test_1_week1_exercise7() {
    let a = vec! [1, 2, 3];
    let b = vec! [1, 2, 4];
    assert_eq!(b, exercise7(a));
}

#[test]
fn test_2_week1_exercise7() {
    let a = vec! [4,3,2,1];
    let b = vec! [4,3,2,2];
    assert_eq!(b, exercise7(a));
}