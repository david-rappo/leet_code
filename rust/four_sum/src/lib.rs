mod solution;

#[cfg(test)]
mod tests {
    use crate::solution;
    use std::collections::HashSet;

    const PARAMETER_1: [&[i32]; 2] = [&[1, 0, -1, 0, -2, 2], &[2, 2, 2, 2, 2]];
    const EXPECTED_RESULT: [&[&[i32]]; 2] = [
        &[&[-2, -1, 1, 2], &[-2, 0, 0, 2], &[-1, 0, 0, 1]],
        &[&[2, 2, 2, 2]],
    ];
    const TARGET: [i32; 2] = [0, 8];

    fn create_sorted_vec(slice: &[i32]) -> Vec<i32> {
        let mut v = vec![];
        v.reserve(slice.len());
        v.extend(slice);
        v.sort();
        v
    }

    fn check(result: &[Vec<i32>], expected_result: &[&[i32]]) -> bool {
        if result.len() != expected_result.len() {
            return false;
        }

        let mut hash_set = HashSet::new();
        for result_vec in result.iter() {
            for (index, expected_result_vec) in expected_result.iter().enumerate() {
                let sorted_result_vec = create_sorted_vec(result_vec);
                let sorted_expected_result_vec = create_sorted_vec(expected_result_vec);
                if sorted_result_vec == sorted_expected_result_vec {
                    let insert_result = hash_set.insert(index);
                    if !insert_result {
                        return false;
                    }

                    break;
                }
            }
        }

        hash_set.len() == expected_result.len()
    }

    #[test]
    fn test_solution_0() {
        let numbers = PARAMETER_1[0].to_owned();
        let target = TARGET[0];
        let expected_result = EXPECTED_RESULT[0];
        let result = solution::four_sum(numbers, target);
        let check_result = check(&result, expected_result);
        assert!(check_result);
    }

    #[test]
    fn test_solution_1() {
        let numbers = PARAMETER_1[1].to_owned();
        let target = TARGET[1];
        let expected_result = EXPECTED_RESULT[1];
        let result = solution::four_sum(numbers, target);
        let check_result = check(&result, expected_result);
        assert!(check_result);
    }

    #[test]
    fn test_solution_gold_0() {
        let numbers = PARAMETER_1[0].to_owned();
        let target = TARGET[0];
        let expected_result = EXPECTED_RESULT[0];
        let result = solution::four_sum_gold(numbers, target);
        // TODO: Does not work.
        println!("test_solution_gold_0: {:?}", result);
        let check_result = check(&result, expected_result);
        assert!(check_result);
    }

    #[test]
    fn test_solution_gold_1() {
        let numbers = PARAMETER_1[1].to_owned();
        let target = TARGET[1];
        let expected_result = EXPECTED_RESULT[1];
        let result = solution::four_sum_gold(numbers, target);
        // TODO: Does not work.
        println!("test_solution_gold_1: {:?}", result);
        let check_result = check(&result, expected_result);
        assert!(check_result);
    }
}
