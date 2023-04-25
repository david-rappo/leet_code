mod solution;

#[cfg(test)]
mod tests {
    use crate::solution;

    fn create_sorted_vec(slice: &[i32]) -> Vec<i32> {
        let mut v = vec![];
        v.reserve(slice.len());
        v.extend(slice);
        v.sort();
        v
    }

    // TODO: Check this.
    fn check(result: &[Vec<i32>], expected_result: &[Vec<i32>]) -> bool {
        if result.len() != expected_result.len() {
            return false;
        }

        let mut count = 0;
        for result_vec in result.iter() {
            for expected_result_vec in expected_result.iter() {
                let sorted_result_vec = create_sorted_vec(result_vec);
                let sorted_expected_result_vec = create_sorted_vec(expected_result_vec);
                if sorted_result_vec == sorted_expected_result_vec {
                    count += 1;
                    break;
                }
            }
        }

        count == expected_result.len()
    }

    #[test]
    fn test_solution_example_1() {
        let numbers = vec![1, 0, -1, 0, -2, 2];
        let target = 0;
        let expected_result = vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]];
        let result = solution::four_sum(numbers, target);
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
