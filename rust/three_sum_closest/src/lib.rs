mod solution;

#[cfg(test)]
mod tests {
    use crate::solution;

    // TODO: EXPECTED_RESULT[2] does not seem right.
    const PARAMETER_0: [&[i32]; 3] = [&[-1, 2, 1, -4], &[0, 0, 0], &[4, 0, 5, -5, 3, 3, 0, -4, -5]];
    const PARAMETER_1: [i32; 3] = [1, 1, -2];
    const EXPECTED_RESULT: [i32; 3] = [2, 0, -2];

    #[test]
    fn it_works() {
        for index in 0..PARAMETER_0.len() {
            let parameter_0 = PARAMETER_0[index];
            let parameter_1 = PARAMETER_1[index];
            let result = solution::three_sum_closest(parameter_0.to_owned(), parameter_1);
            let expected_result = EXPECTED_RESULT[index];
            assert_eq!(result, expected_result);
        }
    }
}
