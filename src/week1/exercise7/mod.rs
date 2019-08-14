pub fn excel_sheet_column_number(input: &str) -> i32 {

    input.chars().rev().enumerate().fold(0, |acc, (index, x)| {
        acc + 26i32.pow(index as u32) * letter_to_number(x)
    })

}

fn letter_to_number(a: char) -> i32 {
    a.to_ascii_uppercase() as i32 - 64
}
