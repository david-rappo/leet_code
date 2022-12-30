#[allow(dead_code)]
pub fn max_area(height: Vec<i32>) -> i32 {
    if height.len() == 2 {
        let left = (0, height[0]);
        let right = (1, height[1]);
        return calculate_area(left, right);
    }

    let mut result = 0;
    for index in 1..height.len() - 1 {
        let left = find_tallest(0, index, &height);
        let right = find_tallest(index + 1, height.len(), &height);
        let area = calculate_area(left, right);
        result = i32::max(result, area);
    }

    result
}

fn find_tallest(begin_index: usize, end_index: usize, heights: &[i32]) -> (usize, i32) {
    let mut result = (begin_index, heights[begin_index]);
    for (index, height) in heights.iter().enumerate().take(end_index).skip(begin_index) {
        if *height > result.1 {
            result = (index, *height);
        }
    }

    result
}

fn calculate_area(left: (usize, i32), right: (usize, i32)) -> i32 {
    let x = right.0 - left.0;
    let h = i32::min(left.1, right.1);
    x as i32 * h
}

#[cfg(test)]
mod tests {
    const PARAMETER_1: [&[i32]; 1] = [&[1, 8, 6, 2, 5, 4, 8, 3, 7]];
    const EXPECTED_RESULT: [i32; 1] = [49];

    #[test]
    fn test_solution() {}
}
