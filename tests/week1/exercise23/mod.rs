use onboarding_rust::week1::exercise23::number_of_islands;

#[test]
fn exercise_23_1() {
    let islas = vec![
        vec!['1', '1', '1', '0', '1'],
        vec!['1', '1', '0', '1', '1'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ];
    let num_islas = 3;
    assert_eq!(num_islas, number_of_islands(islas));
}

#[test]
fn exercise_23_2() {
    let islas = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];
    let num_islas = 1;
    assert_eq!(num_islas, number_of_islands(islas));
}

#[test]
fn exercise_23_3() {
    let islas = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ];
    let num_islas = 3;
    assert_eq!(num_islas, number_of_islands(islas));
}

#[test]
fn exercise23_23_4() {
    let islas = vec![
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '0', '1', '0', '1'],
        vec!['1', '1', '1', '0', '1'],
        vec!['1', '1', '1', '0', '1'],
    ];
    let num_islas = 1;
    assert_eq!(num_islas, number_of_islands(islas));
}

#[test]
fn exercise23_23_5() {
    let islas = vec![
        vec!['1', '1', '1'],
        vec!['0', '1', '0'],
        vec!['1', '1', '1'],
    ];
    // let islas = vec![
    //     vec!['1', '1', '1', '1', '1'],
    //     vec!['1', '0', '1', '0', '1'],
    //     vec!['1', '0', '1', '0', '1'],
    //     vec!['1', '1', '0', '0', '1'],
    //     vec!['1', '0', '1', '1', '1'],
    // ];
    let num_islas = 1;
    assert_eq!(num_islas, number_of_islands(islas));
}
