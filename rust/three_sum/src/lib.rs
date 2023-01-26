mod solution;

#[cfg(test)]
mod tests {
    use crate::solution::three_sum;

    const PARAMETER_0: [&[i32]; 1] = [&[-1, 0, 1, 2, -1, -4]];
    const EXPECTED_RESULT: [&[&[i32]]; 1] = [&[&[-1, -1, 2], &[-1, 0, 1]]];

    fn check(result: Vec<Vec<i32>>, mut expected_result: Vec<&[i32]>) -> bool {
        // TODO:
        for result_vec in result.iter() {}
        false
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
