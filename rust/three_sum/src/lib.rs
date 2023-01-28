mod solution;

#[cfg(test)]
mod tests {
    use crate::solution::three_sum;

    const PARAMETER_0: [&[i32]; 3] = [&[-1, 0, 1, 2, -1, -4], &[0, 1, 1], &[0, 0, 0]];
    const EXPECTED_RESULT: [&[&[i32]]; 3] = [&[&[-1, -1, 2], &[-1, 0, 1]], &[], &[&[0, 0, 0]]];

    fn check(result: Vec<Vec<i32>>, expected_result: Vec<&[i32]>) -> bool {
        let mut count = 0;
        for expected_result_slice in expected_result.into_iter() {
            let mut expected_result_vec = expected_result_slice.to_vec();
            expected_result_vec.sort();
            let result_vec = result.iter().find(|v| {
                let mut sorted_v = (*v).to_owned();
                sorted_v.sort();
                sorted_v == expected_result_vec
            });

            match result_vec {
                Some(_) => {
                    count += 1;
                }
                None => {
                    return false;
                }
            }
        }

        count == result.len()
    }

    #[test]
    fn test_solution() {
        for index in 0..PARAMETER_0.len() {
            let parameter_0 = PARAMETER_0[index];
            let result = three_sum(parameter_0.to_vec());
            let expected_result = EXPECTED_RESULT[index].to_vec();
            assert!(check(result, expected_result));
        }
    }
}
