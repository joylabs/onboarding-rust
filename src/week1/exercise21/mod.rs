pub fn find_circle_num(input: Vec<Vec<i32>>) -> i32 {
    let mut friends_relation = input.clone();

    (0..friends_relation.len()).fold(0, |circles, i| {
        circles
            + (0..friends_relation[0].len()).fold(0, |circle, j| {
                if friends_relation[i][j] == 1 {
                    searching_circle(&mut friends_relation, i, j);
                    println!("i: {}, j: {}, \n {:?}", i, j, friends_relation);
                    circle + 1
                } else {
                    println!("i: {}, j: {}, \n {:?}", i, j, friends_relation);
                    circle
                }
            })
    })
}
fn searching_circle(friends_relation: &mut Vec<Vec<i32>>, i: usize, j: usize) {

    println!("S i: {}, j: {}, \n {:?}", i, j, friends_relation);

    friends_relation[i][j] = 0;

    (0..friends_relation[0].len()).for_each(|j2| {
        if friends_relation[j][j2] == 1 {
            searching_circle(friends_relation, j, j2);
        }
    });

    (0..friends_relation[0].len()).for_each(|j2| {
        if friends_relation[i][j2] == 1 {
            searching_circle(friends_relation, i, j2);
        }
    });
}