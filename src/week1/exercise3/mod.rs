pub fn flip_and_invert_image(input: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut inverted_matrix = vec![];

    for vector in &input {
        let mut last_index = input[0].len();
        let mut inverted_vector = vec![];
        for _element in vector {
            last_index -= 1;
            inverted_vector.push(1 - vector[last_index]);
        }
        inverted_matrix.push(inverted_vector);
    }
    inverted_matrix
}