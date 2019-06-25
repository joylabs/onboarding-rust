// friend-circle exercise
pub fn exercise22(input: Vec<Vec<i32>>) -> i32 {

    let mut uniond_find = FindFrienCirlce::initialice_union_find(input.len() as i32);

    input.into_iter().enumerate().for_each(|(a, row)| {
        find_union(a, row, &mut uniond_find);
    });

    uniond_find.circles
}

pub struct FindFrienCirlce {
    index: Vec<i32>,
    circles: i32,
}

impl FindFrienCirlce {

    fn initialice_union_find(matrix_order: i32) -> FindFrienCirlce {
        FindFrienCirlce {
            index: (0..matrix_order).map(|x| x).collect(),
            circles: matrix_order,
        }
    }
    fn find(&mut self, mut position: usize) -> usize {
        let friend = self.index[position] as usize;
        if position != friend {
            self.index[position as usize] = self.index[position as usize] as i32;
            position = friend;
        }
        position
    }
}

pub fn find_union(a: usize, row: Vec<i32>, friend_circle: &mut FindFrienCirlce) {

    row.iter().enumerate().for_each(|(b, element)| {
        let a = friend_circle.find(a);
        let b = friend_circle.find(b);
        if *element == 1 && a != b {
            friend_circle.index[a] = b as i32;
            friend_circle.circles -= 1;
        }
    });
}

