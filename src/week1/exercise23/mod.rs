
pub fn number_of_islands(grid: Vec<Vec<char>>) -> i32 {

    let mut vector_lands: Vec<(i32, i32)> = grid
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
    while !vector_lands.is_empty() {
        islas.push(vector_lands.remove(0));
        recursiva_searching_sorrounded_lands(*islas.last().unwrap(), &mut vector_lands);
    }

    islas.len() as i32
}

fn recursiva_searching_sorrounded_lands(isla: (i32, i32), lands: &mut Vec<(i32, i32)>) {

    let mut send;
    let le = lands.len();

    //  looking horizontal left
    for i in 0..le {
        if isla.0 == lands[i].0 && isla.1 == lands[i].1 - 1 {
            send = lands.remove(i);
            recursiva_searching_sorrounded_lands(send, lands);
            break;
        }
    }

    let le = lands.len();
    //looking vertical up
    for i in 0..le {
        if isla.1 == lands[i].1 && isla.0 == lands[i].0 - 1 {
            send = lands.remove(i);
            recursiva_searching_sorrounded_lands(send, lands);
            break;
        }
    }

    let le = lands.len();
    // looking horizontal right
    for i in 0..le {
        if isla.0 == lands[i].0 && isla.1 == lands[i].1 + 1 {
            send = lands.remove(i);
            recursiva_searching_sorrounded_lands(send, lands);
            break;
        }
    }
    // looking vertical down
    let le = lands.len();
    for i in 0..le {
        if isla.1 == lands[i].1 && isla.0 == lands[i].0 + 1 {
            send = lands.remove(i);
            recursiva_searching_sorrounded_lands(send, lands);
            break;
        }
    }
}