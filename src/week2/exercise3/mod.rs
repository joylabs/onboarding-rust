pub fn happy_numbers(n: i32) -> bool {
    let depth = 0;
    is_happy(n, depth).is_ok()
}
fn is_happy(mut input: i32, mut depth: i64) -> Result<(()), &'static str> {
    let mut total: i32 = 0;
    while input != 0 {
        total += (input % 10).pow(2);
        input /= 10;
    }
    input = total;
    depth += 1;
    if depth > 1000 {
        return Err("FAIL");
    } else if input != 1 {
        is_happy(input, depth)?;
    }

    Ok(())
}
