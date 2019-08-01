use onboarding_rust::week2::exercise23::find_words;

#[test]
fn test_week2_exercise23_example1() {
    let board = vec![
        vec!['o','a','a','n'],
        vec!['e','t','a','e'],
        vec!['i','h','k','r'],
        vec!['i','f','l','v']
    ];
    let words = vec!["oath".to_string(),"pea".to_string(),"eat".to_string(),"rain".to_string()];
    let expected = vec!["eat".to_string(),"oath".to_string()];
    assert_eq!(expected, find_words(board, words));
}

