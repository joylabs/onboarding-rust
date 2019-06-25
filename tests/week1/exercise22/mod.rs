use onboarding_rust::week1::exercise22::is_friend_circle;

#[test]
fn test_1_week1_exercise22() {
    let a = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
    let b = 2;
    assert_eq!(b, is_friend_circle(a));
}

#[test]
fn test_2_week1_exercise22() {
    let a = vec![vec![1, 1, 0], vec![1, 1, 1], vec![0, 1, 1]];
    let b = 1;
    assert_eq!(b, is_friend_circle(a));
}

#[test]
fn test_3_week1_exercise22() {
    let a = vec![
        vec![1, 0, 0, 0],
        vec![0, 1, 0, 0],
        vec![0, 0, 1, 0],
        vec![0, 0, 0, 1],
    ];
    let b = 4;
    assert_eq!(b, is_friend_circle(a));
}