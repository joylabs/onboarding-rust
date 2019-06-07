pub fn exercise7(mut v: Vec<i32>) -> Vec<i32> {
    let size = v.len() - 1;
    
    let add_to_last_position = v.get(size).unwrap() + 1;

    if add_to_last_position < 10 {
        v[size] = v.get(size).unwrap() + 1;
        return v;
    }
    let my_char: Vec<char> = add_to_last_position.to_string().chars().collect();
    v[size] = my_char[0].to_digit(10).unwrap() as i32;
    v.push(my_char[1].to_digit(10).unwrap() as i32);
    v
}