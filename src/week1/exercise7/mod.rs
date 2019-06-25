const OFFSET: i32 = 'A' as i32 - 1;
const BASE: i32 = 26;
pub fn excel_sheet_column_number(column: &str) -> i32 {
    let characters = column.chars().collect::<Vec<char>>();
    characters
        .iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (*c as i32 - OFFSET) * BASE.pow(i as u32)).sum()

}
