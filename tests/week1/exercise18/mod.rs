use onboarding_rust::week1::exercise18::exercise18;

#[test]
fn test_1_week1_exercise18(){
    let a = vec![2,2,1];
    let b = 1;
    assert_eq!(b,exercise18(a));
}

#[test]
fn test_2_week1_exercise18(){
    let a = vec![4,1,2,1,2];
    let b = 4;
    assert_eq!(b,exercise18(a));
}