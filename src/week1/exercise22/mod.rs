pub fn exercise22(input: Vec<Vec<i32>>) -> i32 {
    let mut uniond_find = initialice_union_find(3);
    let rows = input.len();
    let columns = input[0].len();

    for i in 0..rows {
        for j in 0..columns {
            if input[i][j] == 1 {
                uniond_find.union(i as i32, j as i32);
            }
        }
    }
    uniond_find.circles as i32

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

    fn find(&mut self, mut element: i32) -> i32 {
        let mut parent = self.username[element as usize] as i32;
        while element != parent {
            let grandparent = self.username[element as usize] as i32;
            self.username[element as usize] = grandparent;
            element = parent;
            parent = grandparent;
        }
        element
    }

    pub fn union(&mut self, a: i32, b: i32) -> bool {
        let a = self.find(a);
        let b = self.find(b);

        if a == b {
            return false;
        }

        if a != b {
            self.username[a as usize] = b;
            println!("circle a {}", a);
            self.circles -= 1;
        }
        true
    }
}

