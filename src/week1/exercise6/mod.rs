trait Tuple {
        fn prepend_one(&self) -> Vec<i32>;
        fn carry(y: Self, x: i32) -> Self;
}

impl Tuple for (i32, Vec<i32>) {
        fn prepend_one(&self) -> Vec<i32> {
                let (a, mut b) = self.to_owned();
                if a == 1 {
                        b.push(1);
                }
                b.reverse();
                b
        }

        fn carry(y: Self, x: i32) -> (i32, Vec<i32>) {
                match y {
                        (1, mut v) => {
                                if x == 9 {
                                        v.push(0);
                                        (1, v)
                                } else {
                                        v.push(x + 1);
                                        (0, v)
                                }
                        }
                        (_, mut v) => {
                                v.push(x);
                                (0, v)
                        }
                }

        }
}


pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        digits.iter()
                .rfold((1 as i32, Vec::new()), |y, &x| Tuple::carry(y, x))
                .prepend_one()
}
