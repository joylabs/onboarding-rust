pub fn longest_common_prefix(mut input: Vec<&str>) -> String {

        let mut flag = true;
        input.sort();
        if input.is_empty() {
                return "".to_string();
        }
        input[0].chars()
                .zip(input.last().unwrap().chars())
                .filter(|(x, y)| {
                        if x != y {
                                flag = false;
                        }
                        x == y && flag
                })
                .map(|(x, _)| x)
                .collect::<String>()
}