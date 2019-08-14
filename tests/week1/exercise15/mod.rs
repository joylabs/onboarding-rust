use onboarding_rust::week1::exercise15::number_complement;

#[test]
fn test_week1_exercise15_example1() {

    let input = 5;
    let expected = 2;
    assert_eq!(expected, number_complement(input));
}

#[test]
fn test_week1_exercise15_example2() {

    let input = 8;
    let expected = 7;
    assert_eq!(expected, number_complement(input));
}

#[test]
fn test_week1_exercise15_example3() {

    let input = 0;
    let expected = 1;
    assert_eq!(expected, number_complement(input));
}


#[test]
fn test_week1_exercise15_example4() {

    let input = 127;
    let expected = 0;
    assert_eq!(expected, number_complement(input));
}
