use onboarding_rust::week1::exercise4::self_dividing_numbers;

#[test]
fn test_week1_exercise4_example1() {

    let input = vec![1, 22];
    let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22];
    assert_eq!(expected, self_dividing_numbers(input));
}

#[test]
fn test_week1_exercise4_example2() {

    let input = vec![66, 708];
    let expected = vec![
        66, 77, 88, 99, 111, 112, 115, 122, 124, 126, 128, 132, 135, 144, 155, 162, 168, 175, 184,
        212, 216, 222, 224, 244, 248, 264, 288, 312, 315, 324, 333, 336, 366, 384, 396, 412, 424,
        432, 444, 448, 488, 515, 555, 612, 624, 636, 648, 666, 672,
    ];
    assert_eq!(expected, self_dividing_numbers(input));
}