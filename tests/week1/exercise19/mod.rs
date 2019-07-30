use onboarding_rust::week1::exercise19::get_two_sum;

#[test]
fn test_week1_exercise19_example1() {
    let input = vec![2, 7, 11, 15];
    let target = 9;
    let expected = vec![0, 1];
    assert_eq!(expected, get_two_sum(input, target));
}

#[test]
fn test_week1_exercise19_example2() {
    let input = vec![3, 2, 3];
    let target = 6;
    let expected = vec![0, 2];
    assert_eq!(expected, get_two_sum(input, target));
}

#[test]
fn test_week1_exercise19_example3() {
    let input = vec![0, 4, 3, 0];
    let target = 0;
    let expected = vec![0, 3];
    assert_eq!(expected, get_two_sum(input, target));
}

#[test]
fn test_week1_exercise19_example4() {
    let input = vec![-1, -2, -3, -4, -5];
    let target = -8;
    let expected = vec![2, 4];
    assert_eq!(expected, get_two_sum(input, target));
}

#[test]
fn test_week1_exercise19_example5() {
    let input = vec![-3, 4, 3, 90];
    let target = 0;
    let expected = vec![0, 2];
    assert_eq!(expected, get_two_sum(input, target));
}