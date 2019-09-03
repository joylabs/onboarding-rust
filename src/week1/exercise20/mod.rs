pub fn is_anagram(input1: String, input2: String) -> bool {
    let mut in1 = input1.into_bytes();
    let mut in2 = input2.into_bytes();
    in1.sort();
    in2.sort();

    in1 == in2
}
