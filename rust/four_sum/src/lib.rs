mod solution;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::solution;

    fn create_hash_set(slice: &[i32]) -> HashSet<i32> {
        let mut hash_set = HashSet::new();
        for x in slice {
            hash_set.insert(*x);
        }

        hash_set
    }

    fn check(result: &[Vec<i32>], expected_result: &[Vec<i32>]) -> bool {
        if result.len() != expected_result.len() {
            return false;
        }

        for (index, result_vec) in result.iter().enumerate() {
            let result_hash_set = create_hash_set(result_vec);
            let expected_result_slice = &expected_result[index];
            let expected_result_hash_set = create_hash_set(expected_result_slice);
            let is_result_subset = result_hash_set.is_subset(&expected_result_hash_set);
            if result_hash_set.len() != expected_result_hash_set.len() {
                return false;
            }

            if !is_result_subset {
                return false;
            }
        }

        true
    }

    #[test]
    fn test_solution_example_1() {
        let numbers = vec![1, 0, -1, 0, -2, 2];
        let target = 0;
        let expected_result = vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]];
        let result = solution::four_sum(numbers, target);
        // TODO:
        println!("{:?}", result);
        let check_result = check(&result, &expected_result);
        assert!(check_result);
    }

    #[test]
    fn test_solution_example_2() {
        let numbers = vec![2, 2, 2, 2, 2];
        let target = 8;
        let expected_result = vec![vec![2, 2, 2, 2]];
        let result = solution::four_sum(numbers, target);
        let check_result = check(&result, &expected_result);
        assert!(check_result);
    }
}
