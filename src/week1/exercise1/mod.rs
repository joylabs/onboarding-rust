pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {

    let mut even: Vec<i32> = Vec::new();
    let mut odd: Vec<i32> = Vec::new();

    for num in &a {
        if num % 2 == 0{
            even.push(*num);
        } else if num % 2 != 0 {
            odd.push(*num);
        }
    }

    [&even[..], &odd[..]].concat()
}