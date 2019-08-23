pub fn peak_index_in_mountain_array(a: Vec<i32>) -> i32 {
    let (mut min, mut max) = (0, a.len() - 1);

    while min <= max {
        let middle_index = ((min + max) as f32 / 2.0).ceil() as usize;

        if a[middle_index] > a[middle_index + 1] && a[middle_index] > a[middle_index - 1] {
            return middle_index as i32;
        } else if a[middle_index] < a[middle_index + 1] {
            min = middle_index + 1;
        } else if a[middle_index] > a[middle_index + 1] {
            max = middle_index - 1;
        }
    }

    0
}
