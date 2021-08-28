mod solution;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::solution;

    const TEST_1_NUMBERS: [i32; 4] = [2, 7, 11, 15];
    const TEST_1_TARGET: i32 = 9;

    const TEST_2_NUMBERS: [i32; 3] = [3, 2, 4];
    const TEST_2_TARGET: i32 = 6;

    const TEST_3_NUMBERS: [i32; 2] = [3, 3];
    const TEST_3_TARGET: i32 = 6;

    fn get_test_1_expected_result() -> HashSet<usize> {
        let mut hash_set = HashSet::new();
        hash_set.insert(0);
        hash_set.insert(1);
        hash_set
    }

    fn get_test_2_expected_result() -> HashSet<usize> {
        let mut hash_set = HashSet::new();
        hash_set.insert(1);
        hash_set.insert(2);
        hash_set
    }

    fn get_test_3_expected_result() -> HashSet<usize> {
        let mut hash_set = HashSet::new();
        hash_set.insert(0);
        hash_set.insert(1);
        hash_set
    }

    fn check(hash_set: &HashSet<usize>, result: &[i32]) -> bool {
        for iter in result.iter() {
            let index = *iter as usize;
            let is_contains = hash_set.contains(&index);
            if !is_contains {
                return false;
            }
        }
        
        // If execution reached this point then every value in result was found
        // in hash_set.
        hash_set.len() == result.len()
    }
    
    #[test]
    fn test_1() {
        let result = solution::two_sum(TEST_1_NUMBERS.to_vec(), TEST_1_TARGET);
        assert_eq!(true, check(&get_test_1_expected_result(), &result));
    }

    #[test]
    fn test_2() {
        let result = solution::two_sum(TEST_2_NUMBERS.to_vec(), TEST_2_TARGET);
        assert_eq!(true, check(&get_test_2_expected_result(), &result));
    }

    #[test]
    fn test_3() {
        let result = solution::two_sum(TEST_3_NUMBERS.to_vec(), TEST_3_TARGET);
        assert_eq!(true, check(&get_test_3_expected_result(), &result));
    }
}