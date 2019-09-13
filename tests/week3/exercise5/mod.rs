use onboarding_rust::week3::exercise5::search;

#[test]
fn test_binary_search_one() {
    let nums = vec![-1,0,3,5,9,12];
    let target = 9;
    let output = 4;
    assert_eq!(output,search(nums,target));
}

#[test]
fn test_binary_search_two() {
    let nums = vec![-1,0,3,5,9,12];
    let target = 2;
    let output = -1;
    assert_eq!(output,search(nums,target));
}