mod solution;

#[cfg(test)]
mod tests {
    use crate::solution;

    const PARAMETER_1: [&[i32]; 2] = [&[1, 2, 3, 4, 5, 6, 7, 8],
        &[1, 3, 7, 11, 12, 14, 18]];
    const EXPECTED_RESULT: [i32; 2] = [5, 3];
    
    #[test]
    fn test() {
        for (index, element) in PARAMETER_1.iter().enumerate() {
            let result = solution::longest_fibonacci_subsequence(element.to_vec());
            assert_eq!(result, EXPECTED_RESULT[index]);
        }
    }
}
