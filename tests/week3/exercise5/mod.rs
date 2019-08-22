use onboarding_rust::week3::exercise5::partition_labels;

#[test]
fn test_week3_exercise5_example1() {
    let input = "ababcbacadefegdehijhklij".to_owned();
    let expected = vec![9,7,8];
    assert_eq!(expected, partition_labels(input));
}
