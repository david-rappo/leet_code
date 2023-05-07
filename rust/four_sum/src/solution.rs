use std::{collections::HashMap, collections::HashSet};

// This is the LeetCode solution.
#[allow(dead_code)]
pub fn four_sum_gold(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut sorted = nums;
    sorted.sort();
    k_sum(&sorted, target, 0, 4)
}

// k represents the number of values to sum to make target.
fn k_sum(sorted: &[i32], target: i32, begin_index: usize, k: i32) -> Vec<Vec<i32>> {
    if begin_index >= sorted.len() {
        return vec![];
    }

    let average_value = target / k;
    // If the first value in the range beginning at begin_index is greater than
    // average_value then every value after it is also greater than average_value.
    // (because the values are sorted). Therefore, none of the numbers in the range
    // can possibly sum to target.
    //
    // If the last value in the range is less than average_value then none of the
    // other values in the range can be large enough to sum to target.
    if (sorted[begin_index] > average_value) || (*sorted.last().unwrap() < average_value) {
        return vec![];
    }

    if k == 2 {
        return two_sum(sorted, target, begin_index);
    }

    let mut result: Vec<Vec<i32>> = vec![];
    for index in begin_index..sorted.len() {
        // If either:
        // - Processing the first value in the range
        // - The current value is not equal to the previous value
        if (index == begin_index) || (sorted[index - 1] != sorted[index]) {
            let k_sum_result = k_sum(sorted, target - sorted[index], index + 1, k - 1);
            for sums in k_sum_result.into_iter() {
                let mut v = vec![];
                v.reserve(k as usize);
                v.push(sorted[index]);
                v.extend(sums);
                result.push(v);
            }
        }
    }

    result
}

fn two_sum(sorted: &[i32], target: i32, begin_index: usize) -> Vec<Vec<i32>> {
    vec![]
}

#[allow(dead_code)]
pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let pair_sums = create_pair_sums(&nums);
    let mut result = vec![];
    let mut seen: Vec<HashSet<usize>> = vec![];
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            let number_i = nums[i];
            let number_j = nums[j];
            let sum = number_i + number_j;
            // target = sum + other
            // other = target - sum
            let other = target - sum;
            if pair_sums.contains_key(&other) {
                let vec_pairs = pair_sums.get(&other).unwrap();
                for p in vec_pairs {
                    let hash_set = create_hash_set((i, j), *p);
                    if should_add(&nums, &seen, &hash_set) {
                        let v = create_vec(&nums, &hash_set);
                        seen.push(hash_set);
                        result.push(v);
                    }
                }
            }
        }
    }

    result
}

fn should_add(numbers: &[i32], seen: &[HashSet<usize>], hash_set: &HashSet<usize>) -> bool {
    if hash_set.len() != 4 {
        return false;
    }

    for seen_hash_set in seen {
        if hash_set.is_subset(seen_hash_set) {
            return false;
        }

        let mut seen_vec = create_vec(numbers, seen_hash_set);
        seen_vec.sort();
        let mut vec = create_vec(numbers, hash_set);
        vec.sort();
        if seen_vec == vec {
            return false;
        }
    }

    true
}

fn create_vec(numbers: &[i32], hash_set: &HashSet<usize>) -> Vec<i32> {
    let mut v = vec![];
    v.reserve(hash_set.len());
    for index in hash_set {
        v.push(numbers[*index]);
    }

    v
}

fn create_hash_set(left: (usize, usize), right: (usize, usize)) -> HashSet<usize> {
    let mut hash_set = HashSet::new();
    hash_set.insert(left.0);
    hash_set.insert(left.1);
    hash_set.insert(right.0);
    hash_set.insert(right.1);
    hash_set
}

fn create_pair_sums(numbers: &[i32]) -> HashMap<i32, Vec<(usize, usize)>> {
    let mut hash_map = HashMap::new();
    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            let number_i = numbers[i];
            let number_j = numbers[j];
            let sum = number_i + number_j;
            if let std::collections::hash_map::Entry::Vacant(e) = hash_map.entry(sum) {
                e.insert(vec![(i, j)]);
            } else {
                let vec_pair: &mut Vec<(usize, usize)> = hash_map.get_mut(&sum).unwrap();
                vec_pair.push((i, j));
            }
        }
    }

    hash_map
}
