mod solution;

#[cfg(test)]
mod tests {
    use crate::solution;

    const PARAMETER_1: [i32; 3] = [121, -121, 10];
    const EXPECTED_RESULT: [bool; 3] = [true, false, false];

    #[test]
    fn test() {
        for (index, element) in PARAMETER_1.iter().enumerate() {
            let result = solution::is_palindrome(*element);
            assert_eq!(result, EXPECTED_RESULT[index]);
        }
    }
}
