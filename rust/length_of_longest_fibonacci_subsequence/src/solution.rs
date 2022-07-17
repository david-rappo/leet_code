use std::collections::HashMap;

#[allow(dead_code)]
pub fn longest_fibonacci_subsequence(arr: Vec<i32>) -> i32 {
    // Invalid input.
    if arr.len() < 3 {
        return 0;
    }

    let hash_map = create_hash_map(&arr);
    let end = arr.len() - 1;
    let mut longest = 0u32;
    // For example,
    //        0  1  2  3   4   5   6
    // arr = [1, 3, 7, 11, 12, 14, 18]
    // The range of i is [1, 5]
    // The range of j is [0, 4]
    for i in 1..end {
        for j in 0..i {
            let sum = arr[j] + arr[i];
            let index = hash_map.get(&sum);
            if let Some(k) = index {
                let count = count_elements_in_sequence(&arr,
                    &hash_map,
                    i,
                    *k);
                longest = u32::max(longest, count);
            }
        }
    }
    
    longest as i32
}

fn create_hash_map(arr: &[i32]) -> HashMap<i32, usize> {
    let mut hash_map = HashMap::new();
    for (index, element) in arr.iter().enumerate() {
        hash_map.insert(*element, index);
    }

    hash_map
}

fn count_elements_in_sequence(arr: &[i32],
    hash_map: &HashMap<i32, usize>,
    mut x: usize,
    mut y: usize) -> u32 {
    let mut result = 3;
    loop {
        let sum = arr[x] + arr[y];
        let index = hash_map.get(&sum);
        match index {
            Some(i) => {
                if *i > y {
                    result += 1;
                    x = y;
                    y = *i;
                } else {
                    break;
                }
            },
            None => {
                break;
            }
        }
    }
    
    result
}
