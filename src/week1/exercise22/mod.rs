// friend-circle exercise
pub fn exercise22(input: Vec<Vec<i32>>) -> i32 {

    let mut uniond_find = initialice_union_find(input.len() as i32);
    input
        .into_iter()
        .enumerate()
        .fold(0, |mut circle_number, (a, is_friend)| {
            circle_number =
                is_friend
                    .into_iter()
                    .enumerate()
                    .fold(0, |mut count_circles, (b, element)| {
                        if element == 1 {
                            uniond_find.union(a as i32, b as i32);
                            count_circles = uniond_find.circles;
                        }
                        count_circles
                    });
            circle_number
        })
}


pub struct FindFrienCirlce {
    username: Vec<i32>,
    circles: i32,
}

fn initialice_union_find(num: i32) -> FindFrienCirlce {
    FindFrienCirlce {
        username: (0..num * num).map(|x| x).collect(),
        circles: num,
    }
}
impl FindFrienCirlce {

    fn find(&mut self, mut position: i32) -> i32 {
        let mut friend = self.username[position as usize] as i32;
        while position != friend {
            let search_position = self.username[position as usize] as i32;
            self.username[position as usize] = search_position;
            position = friend;
            friend = search_position;
        }
        position
    }

    pub fn union(&mut self, a: i32, b: i32) {
        let a = self.find(a);
        let b = self.find(b);

        if a != b {
            self.username[a as usize] = b;
            self.circles -= 1;
        }
    }
}

