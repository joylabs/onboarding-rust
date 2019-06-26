pub fn number_of_islands(grid: Vec<Vec<char>>) -> i32 {

    let mut vector_unos: Vec<(i32, i32)> = grid
        .iter()
        .enumerate()
        .map(|(i, c)| {
            c.iter()
                .enumerate()
                .filter(|(_, c)| **c == '1')
                .map(|(j, _)| (i as i32, j as i32))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let mut islas: Vec<(_, _)> = vec![];
    while !vector_unos.is_empty() {
        islas.push(vector_unos.remove(0));
        recursiva(*islas.last().unwrap(), &mut vector_unos);
    }
    //print!("islas: {:?}, resto_islas: {:?} ", islas, vector_unos);
    islas.len() as i32
}

fn recursiva(isla: (i32, i32), unos: &mut Vec<(i32, i32)>) {

    let mut send;
    let le = unos.len();
    for i in 0..le {
        if isla.0 == unos[i].0 && isla.1 == unos[i].1 - 1 {
            send = unos.remove(i);
            recursiva(send, unos);
            break;
        }
    }
    let le = unos.len();
    for i in 0..le {
        if isla.1 == unos[i].1 && isla.0 == unos[i].0 - 1 {
            send = unos.remove(i);
            recursiva(send, unos);
            break;
        }
    }
    let le = unos.len();
    for i in 0..le {
        if isla.0 == unos[i].0 && isla.1 == unos[i].1 + 1 {
            send = unos.remove(i);
            recursiva(send, unos);
            break;
        }
    }
    let le = unos.len();
    for i in 0..le {
        if isla.1 == unos[i].1 && isla.0 == unos[i].0 + 1 {
            send = unos.remove(i);
            recursiva(send, unos);
            break;
        }
    }
}

