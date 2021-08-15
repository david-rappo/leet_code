mod solution;

#[cfg(test)]
mod tests {
    const TEST_1_ARRAY: [i32; 4] = [4, 2, 1, 3];
    const TEST_1_EXPECTED_RESULT: [[i32; 2]; 3] = [[1, 2], [2, 3], [3, 4]];
    
    const TEST_2_ARRAY: [i32; 5] = [1, 3, 6, 10, 15];
    const TEST_2_EXPECTED_RESULT: [[i32; 2]; 1] = [[1, 3]];
    
    const TEST_3_ARRAY: [i32; 8] = [3, 8, -10, 23, 19, -4, -14, 27];
    const TEST_3_EXPECTED_RESULT: [[i32; 2]; 3] = [[-14, -10], [19, 23], [23, 27]];

    fn is_equal(result: &Vec<Vec<i32>>, expected_result: &[[i32; 2]]) -> bool {
        if result.len() != expected_result.len() {
            return false;
        }
        
        for index in 0..result.len() {
            let is_equal_var = result[index] == expected_result[index];
            if !is_equal_var {
                return false
            }
        }

        true
    }
    
    #[test]
    fn test_1() {
        use crate::solution;

        let result = solution::minimum_absolute_difference(TEST_1_ARRAY.to_vec());
        let is_equal_var = is_equal(&result, &TEST_1_EXPECTED_RESULT);
        assert_eq!(true, is_equal_var);
    }

    #[test]
    fn test_2() {
        use crate::solution;

        let result = solution::minimum_absolute_difference(TEST_2_ARRAY.to_vec());
        let is_equal_var = is_equal(&result, &TEST_2_EXPECTED_RESULT);
        assert_eq!(true, is_equal_var);
    }

    #[test]
    fn test_3() {
        use crate::solution;

        let result = solution::minimum_absolute_difference(TEST_3_ARRAY.to_vec());
        let is_equal_var = is_equal(&result, &TEST_3_EXPECTED_RESULT);
        assert_eq!(true, is_equal_var);
    }
}