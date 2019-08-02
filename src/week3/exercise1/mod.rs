use std::collections::HashMap;

pub fn lemonade_change(bills: Vec<i32>) -> bool {
    if bills[0] != 5 {
        return false;
    }

    let mut change_map: HashMap<i32, i32> = HashMap::new();
    change_map.insert(5, 1);
    change_map.insert(10, 0);

    for i in 1..bills.len() {
        if bills[i] == 5 {
            modify_count(&mut change_map, 5, Operation::ADD);

        } else if bills[i] == 10 && *change_map.get(&5).unwrap() >= 1 {
            modify_count(&mut change_map, 5, Operation::SUBSTRACT(1));
            modify_count(&mut change_map, 10, Operation::ADD);

        } else if bills[i] == 20 && (*change_map.get(&5).unwrap() >= 1 && *change_map.get(&10).unwrap() >= 1) {
            modify_count(&mut change_map, 5, Operation::SUBSTRACT(1));
            modify_count(&mut change_map, 10, Operation::SUBSTRACT(1));

        } else if bills[i] == 20 && (*change_map.get(&5).unwrap() >= 3) {
            modify_count(&mut change_map, 5, Operation::SUBSTRACT(3));
        } else {
            return false;
        }
    }

    true
}

fn modify_count(change_map: &mut HashMap<i32, i32>, key: i32, operation: Operation) {
    let count = change_map.entry(key).or_insert(0);
    match operation {
        Operation::ADD => *count += 1,
        Operation::SUBSTRACT(i) => *count -= i,
    }
}

enum Operation {
    ADD,
    SUBSTRACT(i32),
}