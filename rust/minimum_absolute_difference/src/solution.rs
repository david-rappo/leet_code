// Module: solution

// Example
// arr = 4, 2, 1, 3
// arr_sorted = 1, 2, 3, 4
// [1,2], [2,3], [3,4]
#[allow(dead_code)]
pub fn minimum_absolute_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
    assert!(arr.len() >= 2);
    let mut numbers = arr.to_owned();
    numbers.sort_unstable();
    let mut vec_pairs = vec![vec![numbers[0], numbers[1]]];
    let mut best_abs_diff = absolute_difference(numbers[1], numbers[0]);
    let mut index = 1;
    let count = arr.len() - 1;
    while index < count {
        let x = numbers[index];
        let y = numbers[index + 1];
        let abs_diff = absolute_difference(x, y);
        if abs_diff == best_abs_diff {
            vec_pairs.push(vec![x, y]);
        }
        
        /*
        if abs_diff < best_abs_diff {
            best_abs_diff = abs_diff;
            vec_pairs.clear();
            vec_pairs.push(vec![x, y]);
        } else if abs_diff == best_abs_diff {
            vec_pairs.push(vec![x, y]);
        }
        */

        index += 1;
    }
    
    vec_pairs
}

fn absolute_difference(x: i32, y: i32) -> i32 {
    i32::abs(y - x)
}