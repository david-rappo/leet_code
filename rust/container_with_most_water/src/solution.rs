#[allow(dead_code)]
pub fn max_area(height: Vec<i32>) -> i32 {
    if height.is_empty() {
        panic!();
    }

    let mut result = 0;
    let mut left_index = 0;
    let mut right_index = height.len() - 1;
    while left_index < right_index {
        let left_height = height[left_index];
        let left = (left_index, left_height);
        let right_height = height[right_index];
        let right = (right_index, right_height);
        let area = calculate_area(&left, &right);
        result = i32::max(result, area);
        if left_height <= right_height {
            left_index += 1;
        } else {
            right_index = right_index.saturating_sub(1);
        }
    }

    result
}

fn calculate_area(left: &(usize, i32), right: &(usize, i32)) -> i32 {
    debug_assert!(right.0 > left.0);
    let x = right.0 - left.0;
    let h = i32::min(left.1, right.1);
    x as i32 * h
}

#[cfg(test)]
mod tests {
    const PARAMETER_1: [&[i32]; 3] = [&[1, 8, 6, 2, 5, 4, 8, 3, 7], &[1, 1], &[1, 2, 1]];
    const EXPECTED_RESULT: [i32; 3] = [49, 1, 2];

    /*
        #
    #   #   #
    # _ # _ #
      1   2 // Tallest pillar to left
      2   1 // Tallest pillar to right
    */

    #[test]
    fn test_solution() {
        use crate::solution::max_area;

        assert_eq!(PARAMETER_1.len(), EXPECTED_RESULT.len());
        for index in 0..PARAMETER_1.len() {
            let parameter_1 = PARAMETER_1[index];
            let expected_result = EXPECTED_RESULT[index];
            let result = max_area(parameter_1.to_owned());
            assert_eq!(result, expected_result);
        }
    }
}
