use onboarding_rust::week1::exercise24::sorrounded_regions;


#[test]
fn one_sorrounded_regions_e24() {
    let mut region = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];

    let expected = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];

    sorrounded_regions(&mut region);

    assert_eq!(expected, region);
}

#[test]
fn two_sorrounded_regions_e24() {
    let mut region = vec![
        vec!['O', 'X', 'X', 'X', 'X', 'O', 'O', 'X'],
        vec!['O', 'O', 'O', 'O', 'X', 'X', 'X', 'X'],
        vec!['X', 'X', 'O', 'X', 'X', 'O', 'O', 'X'],
        vec!['O', 'O', 'X', 'X', 'X', 'X', 'O', 'X'],
        vec!['X', 'X', 'X', 'O', 'X', 'O', 'O', 'O'],
        vec!['X', 'O', 'X', 'O', 'X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'O', 'O', 'O', 'X', 'X'],
        vec!['X', 'O', 'O', 'O', 'X', 'O', 'X', 'X'],
        vec!['X', 'O', 'X', 'X', 'X', 'O', 'X', 'X'],
        vec!['X', 'O', 'X', 'X', 'X', 'X', 'X', 'X'],
    ];

    let expected = vec![
        vec!['O', 'X', 'X', 'X', 'X', 'O', 'O', 'X'],
        vec!['O', 'O', 'O', 'O', 'X', 'X', 'X', 'X'],
        vec!['X', 'X', 'O', 'X', 'X', 'O', 'O', 'X'],
        vec!['O', 'O', 'X', 'X', 'X', 'X', 'O', 'X'],
        vec!['X', 'X', 'X', 'O', 'X', 'O', 'O', 'O'],
        vec!['X', 'X', 'X', 'O', 'X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'O', 'O', 'O', 'X', 'X'],
        vec!['X', 'O', 'O', 'O', 'X', 'O', 'X', 'X'],
        vec!['X', 'O', 'X', 'X', 'X', 'O', 'X', 'X'],
        vec!['X', 'O', 'X', 'X', 'X', 'X', 'X', 'X'],
    ];


    sorrounded_regions(&mut region);

    assert_eq!(expected, region);
}

#[test]
fn three_sorrounded_regions_e24() {
    let mut region = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'O'],
        vec!['X', 'X', 'X', 'X'],
    ];

    let expected = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'O'],
        vec!['X', 'X', 'X', 'X'],
    ];

    sorrounded_regions(&mut region);
    assert_eq!(expected, region);

}