mod solution;

#[cfg(test)]
mod tests {
    use crate::solution;

    const PARAMETER_1: [i32; 3] = [2, 3, 4];
    const EXPECTED_RESULT: [i32; 3] = [1, 2, 3];
    
    #[test]
    fn test_fibonacci_number() {
        for index in 0..PARAMETER_1.len() {
            let result = solution::fibonacci_number(PARAMETER_1[index]);
            assert_eq!(result, EXPECTED_RESULT[index]);
        }
    }
}
