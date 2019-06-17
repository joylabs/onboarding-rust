use std::collections::HashSet;

pub fn exercise19(x: i32) -> bool {
    let mut number = x;
    let mut seen = HashSet::new();

    while !seen.contains(&number){
        println!("{}",x );
        seen.insert(number);
        number = is_happy(number);
        if number == 1{return true} 
    }
    false
}

fn is_happy(x:i32)->i32{
    let mut squares_sum = 0;
    for i in x.to_string().chars() {
        squares_sum += i.to_digit(10).unwrap().pow(2);
    }
    squares_sum as i32
}