pub fn flip_and_invert_image(input: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    input.iter().map(|vector| {
        vector
            .iter()
            .rev()
            .map(|element| invert_element(*element))
            .collect()
    }).collect()
}

fn invert_element(element: i32) -> i32 {
    1 - element
}