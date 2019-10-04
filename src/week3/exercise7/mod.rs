
pub fn peak_index_in_mountain_array_2(a: Vec<i32>) -> i32 {
	a.iter()
		.enumerate()
		.skip(1)
		.find(|(i, item)| Some(i).map_or(false, |j| a[j - 1] < **item && a[j + 1] < **item))
		.map(|(i, _)| i as i32)
		.unwrap()
}
//Binary search 
pub fn peak_index_in_mountain_array(a: Vec<i32>) -> i32 {
	let mut max = a.len();
	let mut min = 0;
	while min < max {
		let position = (max + min) / 2;
		if a[position] < a[position + 1] {
			min = position + 1;
		} else {
			max = position;
		}
	}
	min as i32
}
