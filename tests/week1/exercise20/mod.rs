use onboarding_rust::week1::exercise20::exercise20;


#[test]
fn test_1_week1_exercise20() {
    let a = vec![2, 7, 11, 15];
    let b = vec![0, 1];
    let c = 9;
    assert_eq!(b, exercise20(a,c));
}

#[test]
fn test_2_week1_exercise20() {
    let a = vec![3,5,6,9,14,16];
    let b = vec![4,5];
    let c = 30;
    assert_eq!(b, exercise20(a,c));
}
