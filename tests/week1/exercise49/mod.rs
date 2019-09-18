use onboarding_rust::week1::exercise49::partition_labels;

#[test]
fn test_partition_labels_three() {
    let input = "ababcbacadefegdehijhklij".to_string();
    let output = vec![9, 7, 8];
    assert_eq!(output, partition_labels(input));
}

#[test]
fn test_partition_labels_one() {
    let input = "aaa".to_string();
    let output = vec![3];
    assert_eq!(output, partition_labels(input));
}

#[test]
fn test_partition_labels_three_single_characters() {
    let input = "abc".to_string();
    let output = vec![1, 1, 1];
    assert_eq!(output, partition_labels(input));
}