use std::{collections::HashMap, collections::HashSet};

#[allow(dead_code)]
pub fn four_sum_gold(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    if nums.is_empty() {
        return vec![];
    }

    // Pair sums in the range [0, i) are stored in pair_sums.
    // Pair sums in the range [i + 1, nums.len()) are not.
    // ... i ...
    let mut result = vec![];
    let mut pair_sums = HashMap::new();
    for i in 1..nums.len() - 1 {
        let number_i = nums[i];
        for j in nums.iter().take(i) {
            let sum = number_i + j;
            if let std::collections::hash_map::Entry::Vacant(e) = pair_sums.entry(sum) {
                e.insert(vec![(number_i, *j)]);
            } else {
                let vec_pair: &mut Vec<(i32, i32)> = pair_sums.get_mut(&sum).unwrap();
                vec_pair.push((number_i, *j));
            }
        }

        // For example,
        // Indexes = 0 1 2 3 4 5 6 7 8 9
        // i = 3           *
        // j =               *
        for j in nums.iter().skip(i + 1) {
            let sum = number_i + j;
            // target = sum + other
            // other = target - sum
            let other = target - sum;
            if pair_sums.contains_key(&other) {
                let vec_pairs = pair_sums.get(&other).unwrap();
                for p in vec_pairs {
                    let v = vec![p.0, p.1, number_i, *j];
                    result.push(v);
                }
            }
        }
    }

    result
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
