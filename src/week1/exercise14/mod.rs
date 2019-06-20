pub fn longest_common_prefix(mut input: Vec<&str>) -> String {

        input.sort();
        if input.is_empty() || input[0].is_empty() {
                return "".to_string();
        }
        if input.len() > 1 && input[0] != input[input.len() - 1] {
                let idx = input[0]
                        .chars()
                        .enumerate()
                        .zip(input.last().unwrap().chars())
                        .find(|((_, x), y)| x != y)
                        .map(|((i, _), _)| i)
                        .unwrap_or_else(|| input[0].len());

                return input[0][..idx].to_string();

        }
        input[0].to_string()
}