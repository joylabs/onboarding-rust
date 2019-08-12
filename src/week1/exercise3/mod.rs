pub fn flip_and_invert_image(input: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut reversed_matrix: Vec<Vec<i32>> = vec![];
    let mut inverted_matrix: Vec<Vec<i32>> = vec![];

    for vector in &input {
        let mut last_index = input[0].len();
        let mut reversed_vector: Vec<i32> = vec![];
        for _element in vector {
            last_index -= 1;
            reversed_vector.push(vector[last_index]);
        }
        reversed_matrix.push(reversed_vector);
    }

    for vector in &reversed_matrix {
        let mut inverted_vector: Vec<i32> = vec![];
        for element in vector {
            if *element == 0 {
                inverted_vector.push(1);
            } else {
                inverted_vector.push(0);
            }

        }
        inverted_matrix.push(inverted_vector);
    }
    inverted_matrix
}