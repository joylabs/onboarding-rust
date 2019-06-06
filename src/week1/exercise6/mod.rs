use inflections::case::{is_lower_case, is_title_case, is_upper_case};

pub fn exercise6(s: &str)->bool{
    match s {
        s if is_title_case(s) => true,
        s if is_lower_case(s) => true,
        s if is_upper_case(s) => true,
        _ => false,
    }
} 