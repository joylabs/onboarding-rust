use onboarding_rust::week1::exercise7::excel_sheet_column_number;

#[test]
fn test_week1_exercise7() {
    let input: &str = "A";
    assert_eq!(1, excel_sheet_column_number(input));
}

#[test]
fn test_week1_exercise7_two_characters() {
    let input: &str = "AB";
    assert_eq!(28, excel_sheet_column_number(input));
}

#[test]
fn test_week1_exercise7_three_characters() {
    let input: &str = "XYZ";
    assert_eq!(16900, excel_sheet_column_number(input));
}