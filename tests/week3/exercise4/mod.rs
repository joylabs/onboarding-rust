use onboarding_rust::week3::exercise4::partition_labels;

#[test]
fn test_partition_labels_one() {
    let s = "ababcbacadefegdehijhklij".to_string();
    let output = vec![9, 7, 8];
    assert_eq!(output, partition_labels(s));
}


#[test]
fn test_partition_labels_two() {
    let s = "eaaaabaaec".to_string();
    let output = vec![9, 1];
    assert_eq!(output, partition_labels(s));
}