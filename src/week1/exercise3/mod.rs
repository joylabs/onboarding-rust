pub fn flipping_image(input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    //Reverse matrix
    input
        .into_iter()
        .map(|row| {
            row.into_iter()
                .rev()
                .map(|element| if element == 1 { 0 } else { 1 })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>()
}
