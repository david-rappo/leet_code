use std::collections::HashMap;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn three_sum_solution_two(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut sorted_numbers = nums;
    sorted_numbers.sort();
    let mut set_results = HashSet::new();
    for index in 0..sorted_numbers.len() {
        let mut j = index + 1;
        let mut k = sorted_numbers.len() - 1;
        while j < k {
            let sum = sorted_numbers[index] + sorted_numbers[j] + sorted_numbers[k];
            if 0 == sum {
                set_results.insert(vec![
                    sorted_numbers[index],
                    sorted_numbers[j],
                    sorted_numbers[k],
                ]);
                j += 1;
                k -= 1;
            } else if sum < 0 {
                j += 1;
            } else {
                k -= 1;
            }
        }
    }

    let mut result = vec![];
    result.reserve(set_results.len());
    for v in set_results.into_iter() {
        result.push(v);
    }

    result
}

#[allow(dead_code)]
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut set_results = HashSet::new();
    let two_sum_to_indexes = create_two_sum_to_indexes(&nums);
    for pair in nums.iter().enumerate() {
        let value = pair.1;
        // value + y = 0
        let y = 0 - value;
        if two_sum_to_indexes.contains_key(&y) {
            let set_indexes: &HashSet<(usize, usize)> = two_sum_to_indexes.get(&y).unwrap();
            let triplets = get_zero_sum_triplets(pair.0, set_indexes, &nums);
            for triplet in triplets.into_iter() {
                set_results.insert(triplet);
            }
        }
    }

    let mut result = vec![];
    result.reserve(set_results.len());
    for v in set_results.into_iter() {
        result.push(v);
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
        debug_assert!(pair.0 < pair.1);
        if (index != pair.0) && (index != pair.1) {
            let mut v = vec![numbers[index], numbers[pair.0], numbers[pair.1]];
            v.sort();
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
