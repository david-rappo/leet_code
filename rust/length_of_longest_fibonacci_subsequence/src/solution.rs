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

#[allow(dead_code)]
pub fn longest_fibonacci_subsequence_dynamic(arr: Vec<i32>) -> i32 {
    // Invalid input.
    if arr.len() < 3 {
        return 0;
    }
    
    let hash_map = create_hash_map(&arr);
    // The horizontal axis of grid is j (columns).
    // The vertical axis of grid is k (rows).
    let mut grid = vec![0; arr.len() * arr.len()];
    let mut result = 0;
    for k in 2..arr.len() {
        for j in 1..k {
            // arr[i] + arr[j] = arr[k]
            // arr[i] = arr[k] - arr[j]
            let element_at_i = arr[k] - arr[j];
            let i = hash_map.get(&element_at_i);
            if let Some(i) = i {
                // The condition: i < j < k must be true.
                if *i < j {
                    // Is there another Fibonacci-like sequence connected to
                    // the Fibonacci-like sequence that was just found?
                    let previous_j = *i;
                    let previous_k = j;
                    // count could be zero because the entire grid is initially
                    // initialized to zero.
                    let count = get_value(&(previous_k, previous_j), arr.len(), &grid);
                    let new_count = count + 1;
                    set_value(&(k, j), arr.len(), new_count, &mut grid);
                    result = u32::max(result, new_count);
                }
            }
        }
    }

    if result > 0 {
        result += 2;
    }
    
    result as i32
}

// position.0: row index
// position.1: column index
fn get_index(position: &(usize, usize), column_count: usize) -> usize {
    position.0 * column_count + position.1
}

// position.0: row index
// position.1: column index
fn get_value(position: &(usize, usize), column_count: usize, grid: &[u32]) -> u32 {
    let index = get_index(position, column_count);
    grid[index]
}

// position.0: row index
// position.1: column index
fn set_value(position: &(usize, usize),
    column_count: usize,
    value: u32,
    grid: &mut [u32]) {
    let index = get_index(position, column_count);
    grid[index] = value;
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
