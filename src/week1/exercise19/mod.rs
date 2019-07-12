pub fn two_sum(input: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = Vec::new();
    'outer: for (i, a) in input.iter().enumerate() {
        for (j, b) in input.iter().enumerate().skip(i + 1) {
            if a + b == target {
                result.push(i as i32);
                result.push(j as i32);
                break 'outer;
            }

        }
    }
    result
}
