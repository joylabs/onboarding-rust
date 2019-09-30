use onboarding_rust::week3::exercise8::array_pair_sum;

#[test]
fn test_array_partition() {
   let input = vec![1,4,3,2];
   let output = 4;
   assert_eq!(output, array_pair_sum(input));
}
