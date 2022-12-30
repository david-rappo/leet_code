#[allow(dead_code)]
pub fn max_area(height: Vec<i32>) -> i32 {
    let tallest_left = calculate_tallest_left(&height);
    let tallest_right = calculate_tallest_right(&height);
    let mut result = 0;
    let count = height.len() - 1;
    for index in 0..count {
        let left = tallest_left[index];
        let right = tallest_right[index];
        let area = calculate_area(left, right);
        result = i32::max(result, area);
    }

    result
}

/*
9
8
7
6
5                   *
4         *         *
3         *       * *
2       * *       * *
1       * *     * * *   *
0       * * * * * * * * * *
        0 1 2 3 4 5 6 7 8 9

        There are ten pillars.
        There are nine positions.
*/

// result[0] is the height of the tallest pillar to the left of position zero. position zero is between pillars zero
// and pillars one.
fn calculate_tallest_left(heights: &[i32]) -> Vec<(usize, i32)> {
    let position_count = heights.len() - 1;
    let mut maximum_height = (0, heights[0]);
    let mut result = Vec::new();
    result.reserve(position_count);
    for (pillar_index, height) in heights.iter().enumerate().take(position_count) {
        if *height > maximum_height.1 {
            maximum_height = (pillar_index, *height);
        }

        result.push(maximum_height);
    }

    result
}

// result[N - 2] is the height of the tallest pillar to the right of position N - 2. position N - 2 is between pillars
// N - 2 and N - 1.
fn calculate_tallest_right(heights: &[i32]) -> Vec<(usize, i32)> {
    let position_count = heights.len() - 1;
    let mut maximum_height = (position_count, *heights.last().unwrap());
    let mut result = vec![maximum_height; position_count];
    let mut result_index = result.len() - 1;
    for (pillar_index, height) in heights.iter().enumerate().rev().take(position_count) {
        if height > &maximum_height.1 {
            maximum_height = (pillar_index, *height);
        }

        result[result_index] = maximum_height;
        result_index = result_index.saturating_sub(1);
    }

    result
}

fn calculate_area(left: (usize, i32), right: (usize, i32)) -> i32 {
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