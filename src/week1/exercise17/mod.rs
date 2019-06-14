pub fn exercise17(a: i32) -> i32 {
    let mut binary = format!("{:b}", a);
    let mut distancia = 0;
    let mut first = binary.find('1');

    while first != None {
        binary = binary.split_at(first.unwrap() + 1).1.to_string();
        first = binary.find('1');
        if let Some(idx) = first {
            let value = idx as i32 + 1;
            if value > distancia {
                distancia = value;
            }
        }
    }
    distancia
}

