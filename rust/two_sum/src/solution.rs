use std::collections::HashMap;

#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // x + y = target
    // y = target - x
    let mut value_to_index = HashMap::new();
    for iter in nums.iter().enumerate() {
        value_to_index.insert(*iter.1, iter.0);
    }

    for iter in nums.iter().enumerate() {
        let y = target - iter.1;
        if value_to_index.contains_key(&y) {
            let first_index = iter.0;
            let second_index = *value_to_index.get(&y).unwrap();
            if first_index != second_index {
                return vec![first_index as i32, second_index as i32];
            }
        }
    }

    vec![]
}