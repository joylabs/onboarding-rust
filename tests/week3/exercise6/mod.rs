use onboarding_rust::week3::exercise6::search;

#[test]
fn test_week3_exercise6_example1() {
    let nums = vec![-1,0,3,5,9,12];
    let target = 9;
    let expected = 4;
    assert_eq!(expected, search(nums, target));
}

#[test]
fn test_week3_exercise6_example2() {
    let nums = vec![-1,0,3,5,9,12];
    let target = 2;
    let expected = -1;
    assert_eq!(expected, search(nums, target));
}

#[test]
fn test_week3_exercise6_example3() {
    let nums = vec![];
    let target = 1;
    let expected = -1;
    assert_eq!(expected, search(nums, target));
}