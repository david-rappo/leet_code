mod solution;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::solution::remove_element;

    const PARAMETER_1: [&[i32]; 2] = [&[3, 2, 2, 3], &[0, 1, 2, 2, 3, 0, 4, 2]];
    const PARAMETER_2: [i32; 2] = [3, 2];
    const EXPECTED_RESULT: [i32; 2] = [2, 5];
    const EXPECTED_RESULT_VEC: [&[i32]; 2] = [&[2, 2], &[0, 0, 1, 3, 4]];

    fn create_hash_set(slice: &[i32]) -> HashSet<i32> {
        let mut hash_set = HashSet::new();
        for x in slice.iter() {
            hash_set.insert(*x);
        }

        hash_set
    }

    fn check_left_contains_right(left: &[i32], right: &[i32]) -> bool {
        let left_hash_set = create_hash_set(left);
        let right_hash_set = create_hash_set(right);
        right_hash_set.is_subset(&left_hash_set)
    }

    #[test]
    fn test_solution() {
        for (index, parameter_1) in PARAMETER_1.iter().enumerate() {
            let parameter_2 = PARAMETER_2[index];
            let slice = *parameter_1;
            let mut copy_parameter_1 = slice.to_owned();
            let result = remove_element(&mut copy_parameter_1, parameter_2);
            assert_eq!(result, EXPECTED_RESULT[index]);
            let expected_result_vec = EXPECTED_RESULT_VEC[index];
            assert!(copy_parameter_1.len() >= expected_result_vec.len());
            let result_slice = &copy_parameter_1[0..expected_result_vec.len()];
            let result_slice_is_subset =
                check_left_contains_right(expected_result_vec, result_slice);
            assert!(result_slice_is_subset);
        }
    }
}
