use std::cmp::max;

pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }

    max(
        get_robbed_money(&nums, Houses::EVEN),
        get_robbed_money(&nums, Houses::ODD),
    )
}

fn get_robbed_money(nums: &[i32], houses_evaluated: Houses) -> i32 {
    match houses_evaluated {
        Houses::EVEN => {
            nums.iter()
                .rev()
                .skip(1)
                .fold((0, 0), |(current, previous), house| {
                    (max(house + previous, current), current)
                })
                .0
        }
        Houses::ODD => {
            nums.iter()
                .skip(1)
                .fold((0, 0), |(current, previous), house| {
                    (max(house + previous, current), current)
                })
                .0
        }
    }
}

enum Houses {
    ODD,
    EVEN,
}
