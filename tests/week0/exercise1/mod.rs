use onboarding_rust::week0::exercise1::overlap_rectangles;

#[test]
fn test_week0_exercise1_example1() {
    let rect1 = vec![0, 0, 2, 2];
    let rect2 = vec![1, 1, 3, 3];
    let expected = true;

    assert_eq!(expected, overlap_rectangles(rect1, rect2));
}

#[test]
fn test_week0_exercise1_example2() {
    let rect1 = vec![0, 0, 1, 1];
    let rect2 = vec![1, 0, 2, 1];
    let expected = false;

    assert_eq!(expected, overlap_rectangles(rect1, rect2));
}
