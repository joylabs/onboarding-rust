use onboarding_rust::week1::exercise3::exercise3;

#[test]
fn test_week1_exercise3_example1() {
    //Input: [[1,1,0],[1,0,1],[0,0,0]]
    //Output: [[1,0,0],[0,1,0],[1,1,1]]

    let input = vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]];
    let expected = vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
    assert_eq!(expected, exercise3(input));
}

#[test]
fn test_week1_exercise3_example2() {
    //Input: [[1,1,0,0],[1,0,0,1],[0,1,1,1],[1,0,1,0]]
    //Output: [[1,1,0,0],[0,1,1,0],[0,0,0,1],[1,0,1,0]]

    let input = vec![
        vec![1, 1, 0, 0],
        vec![1, 0, 0, 1],
        vec![0, 1, 1, 1],
        vec![1, 0, 1, 0],
    ];
    let expected = vec![
        vec![1, 1, 0, 0],
        vec![0, 1, 1, 0],
        vec![0, 0, 0, 1],
        vec![1, 0, 1, 0],
    ];
    assert_eq!(expected, exercise3(input));
}
