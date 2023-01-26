use std::collections::HashMap;
use std::collections::HashSet;

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let two_sum_to_indexes = create_two_sum_to_indexes(&nums);
    for pair in nums.iter().enumerate() {
        let value = pair.1;
        // value + y = 0
        let y = 0 - value;
        if two_sum_to_indexes.contains_key(&y) {
            let set_indexes: &HashSet<(usize, usize)> = two_sum_to_indexes.get(&y).unwrap();
            let mut triplets = get_zero_sum_triplets(pair.0, set_indexes, &nums);
            result.append(&mut triplets);
        }
    }

    result
}

fn get_zero_sum_triplets(
    index: usize,
    set_indexes: &HashSet<(usize, usize)>,
    numbers: &[i32],
) -> Vec<Vec<i32>> {
    let mut result = vec![];
    for pair in set_indexes.iter() {
        debug_assert_ne!(pair.0, pair.1);
        if (index != pair.0) && (index != pair.1) {
            let v = vec![numbers[index], numbers[pair.0], numbers[pair.1]];
            result.push(v);
        }
    }

    result
}

fn create_two_sum_to_indexes(numbers: &[i32]) -> HashMap<i32, HashSet<(usize, usize)>> {
    let mut hash_map = HashMap::new();
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i != j {
                let sum = numbers[i] + numbers[j];
                // (1, 0) and (0, 1) refer to the same elements in numbers. However,
                // if the smallest index is always first in the pair then:
                // (1, 0) => (0, 1)
                // (0, 1) => (0, 1)
                // Inserting (0, 1) into a HashSet twice will only succeed for the
                // first insertion.
                let pair = if i < j { (i, j) } else { (j, i) };
                if hash_map.contains_key(&sum) {
                    let set_indexes: &mut HashSet<(usize, usize)> = hash_map.get_mut(&sum).unwrap();
                    set_indexes.insert(pair);
                } else {
                    let mut set_indexes = HashSet::new();
                    set_indexes.insert(pair);
                    hash_map.insert(sum, set_indexes);
                }
            }
        }
    }

    hash_map
}
