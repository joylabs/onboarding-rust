pub fn find_complement(num: i32) -> i32 {
    let binary_num = format!("{:b}", num);
    let inverted_binary = binary_num.chars().map(invert_element).collect::<String>();
    isize::from_str_radix(&inverted_binary, 2).unwrap() as i32
}

fn invert_element(element: char) -> char {
    match element {
        '1' => '0',
        '0' => '1',
        _ => panic!(),
    }
}
