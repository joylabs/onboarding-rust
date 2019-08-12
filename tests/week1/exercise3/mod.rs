use onboarding_rust::week1::exercise3::flip_and_invert_image;

#[test]
fn test_week1_exercise3_3_by_3_matrix() {
    let input = vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]];
    let output = vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
    assert_eq!(output, flip_and_invert_image(input));
}

#[test]
fn test_week1_exercise3_4_by_4_matrix() {
    let input = vec![
        vec![1, 1, 0, 0],
        vec![1, 0, 0, 1],
        vec![0, 1, 1, 1],
        vec![1, 0, 1, 0],
    ];
    let output = vec![
        vec![1, 1, 0, 0],
        vec![0, 1, 1, 0],
        vec![0, 0, 0, 1],
        vec![1, 0, 1, 0],
    ];
    assert_eq!(output, flip_and_invert_image(input));
}