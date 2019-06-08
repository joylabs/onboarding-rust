pub fn exercise8(s: String)->i32 {
    let mut cont = 0;
    let mut suma = 0;
    let v: i32 = 26;
    for i in s.chars().rev() {
        suma += search_position(i) * v.pow(cont);
        cont += 1;
    }
    suma
}

fn search_position(c: char) -> i32 {
    let v = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    let v = v.into_iter().position(|x| x == c).unwrap() + 1;
    v as i32
}