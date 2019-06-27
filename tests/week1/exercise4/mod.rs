use onboarding_rust::week1::exercise4::self_dividing_numbers;

#[test]
fn test_week1_exercise4() {
    let left: i32 = 1;
    let right: i32 = 22;
    let output = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22];
    assert_eq!(output, self_dividing_numbers(left, right));
}

#[test]
fn test_week1_exercise4_2() {
    let left: i32 = 1;
    let right: i32 = 1000;
    let output = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22, 24, 33, 36, 44, 48, 55, 66, 77, 88, 99, 111,
        112, 115, 122, 124, 126, 128, 132, 135, 144, 155, 162, 168, 175, 184, 212, 216, 222, 224,
        244, 248, 264, 288, 312, 315, 324, 333, 336, 366, 384, 396, 412, 424, 432, 444, 448, 488,
        515, 555, 612, 624, 636, 648, 666, 672, 728, 735, 777, 784, 816, 824, 848, 864, 888, 936,
        999,
    ];
    assert_eq!(output, self_dividing_numbers(left, right));
}
