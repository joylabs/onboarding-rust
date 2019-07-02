struct IndirectFriendCircle {
    index_vector: Vec<i32>,
    circle_count: i32,
}

impl IndirectFriendCircle {
    fn new(matrix_order: i32) -> IndirectFriendCircle {
        IndirectFriendCircle {
            index_vector: (0..matrix_order).map(|x| x).collect(),
            circle_count: matrix_order,
        }
    }

    fn find(&mut self, mut position: usize) -> usize {
        let friend = self.index_vector[position] as usize;
        if position != friend {
            self.index_vector[position] = self.index_vector[position];
            position = friend;
        }
        position
    }
}

pub fn count_friend_circle(input: Vec<Vec<i32>>) -> i32 {
    let matrix_order = input.len() as i32;
    let mut indirect_friend_circle = IndirectFriendCircle::new(matrix_order);

    input.into_iter().enumerate().for_each(|(i, row)| {
        get_one_count(i, row, &mut indirect_friend_circle);
    });

    indirect_friend_circle.circle_count
}

fn get_one_count(i: usize, row: Vec<i32>, indirect_friend_circle: &mut IndirectFriendCircle) {
    row.into_iter().enumerate().for_each(|(j, element)| {
        println!("element {}", element);

        let a = indirect_friend_circle.find(i);
        let b = indirect_friend_circle.find(j);

        if element == 1 && a != b {
            indirect_friend_circle.index_vector[a] = b as i32;
            indirect_friend_circle.circle_count -= 1;
        }
    });
}
