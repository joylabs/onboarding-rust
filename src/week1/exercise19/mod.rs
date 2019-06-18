use std::collections::HashSet;

pub fn exercise19(x: i32) -> bool {
    let mut number = x;
    let mut seen = HashSet::new();

    while !seen.contains(&number){
        seen.insert(number);
        number = is_happy(number);
        if number == 1{return true} 
    }
    false
}

fn is_happy(x:i32)->i32{
    
    x.to_string().chars().fold(0,|mut acc, character|{
         acc += character.to_digit(10).unwrap().pow(2) as i32; acc})
}