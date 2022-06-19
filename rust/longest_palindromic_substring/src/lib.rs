mod solution;

#[cfg(test)]
mod tests {
    use crate::solution;

    const PARAMETER_1: [&str; 3] = ["abaxyzzyxf", "babad", "cbbd"];
    const EXPECTED_RESULTS: [&str; 3] = ["xyzzyx", "bab", "bb"];

    #[test]
    fn test_one() {
        let result = solution::longest_palindrome(PARAMETER_1[0].to_owned());
        assert_eq!(result, EXPECTED_RESULTS[0]);
    }

    #[test]
    fn test_two() {
        let result = solution::longest_palindrome(PARAMETER_1[1].to_owned());
        let other_valid_answer = "aba";
        if result == EXPECTED_RESULTS[1] {
            assert_eq!(result, EXPECTED_RESULTS[1]);
        } else {
            assert_eq!(result, other_valid_answer);
        }
    }

    #[test]
    fn test_three() {
        let result = solution::longest_palindrome(PARAMETER_1[2].to_owned());
        assert_eq!(result, EXPECTED_RESULTS[2]);
    }
}
