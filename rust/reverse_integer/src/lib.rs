mod solution;

#[cfg(test)]
mod tests {
    use crate::solution;

    const PARAMETER_1: [i32; 3] = [123, -123, 120];
    const EXPECTED_RESULT: [i32; 3] = [321, -321, 21];

    #[test]
    fn test_solution() {
        for index in 0..PARAMETER_1.len() {
            let result = solution::reverse(PARAMETER_1[index]);
            assert_eq!(result, EXPECTED_RESULT[index]);
        }
    }
}
