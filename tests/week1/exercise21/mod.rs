use onboarding_rust::week1::exercise21::friend_circles;

#[test]
fn test_week1_friend_circles_two_circles() {
    let input = vec![vec![1,1,0], vec![1,1,0], vec![0,0,1]];
    let output = 2;
    assert_eq!(output, friend_circles(input));
}

#[test]
fn test_week1_friend_circles_one_circle() {
    let input = vec![vec![1,1,0], vec![1,1,1], vec![0,1,1]];
    let output = 1;
    assert_eq!(output, friend_circles(input));
}