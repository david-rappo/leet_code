use std::{collections::HashMap, collections::HashSet};

#[allow(dead_code)]
pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let pair_sums = create_pair_sums(&nums);
    let mut result = vec![];
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            let number_i = nums[i];
            let number_j = nums[j];
            let sum = number_i + number_j;
            // target = sum + other
            // other = target - sum
            let other = target - sum;
            if pair_sums.contains_key(&other) {
                let vec_pair = pair_sums.get(&other).unwrap();
                for p in vec_pair {
                    let hash_set = create_hash_set((i, j), *p);
                    let v = vec![nums[i], nums[j], nums[p.0], nums[p.1]];
                    if (hash_set.len() == 4) && (!result.contains(&v)) {
                        result.push(v);
                    }
                }
            }
        }
    }

    result
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
