mod solution;

#[cfg(test)]
mod tests {
    use crate::solution;

    const PARAMETER_1: [&str; 6] = ["abaxyzzyxf",
        "babad",
        "cbbbd",
        "xxxxx",
        "abaxxxxxreerx",
        "aaaa"];
    const EXPECTED_RESULTS: [&str; 6] = ["xyzzyx",
        "bab",
        "bbb",
        "xxxxx",
        "xreerx",
        "aaaa"];
    
    //
    // solution::longest_palindrome
    //
    
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

    #[test]
    fn test_four() {
        let result = solution::longest_palindrome(PARAMETER_1[3].to_owned());
        assert_eq!(result, EXPECTED_RESULTS[3]);
    }

    #[test]
    fn test_five() {
        let result = solution::longest_palindrome(PARAMETER_1[4].to_owned());
        assert_eq!(result, EXPECTED_RESULTS[4]);
    }

    #[test]
    fn test_six() {
        let result = solution::longest_palindrome(PARAMETER_1[5].to_owned());
        assert_eq!(result, EXPECTED_RESULTS[5]);
    }

    //
    // solution::longest_palindrome_gold
    //

    #[test]
    fn test_one_gold() {
        let result = solution::longest_palindrome_gold(PARAMETER_1[0].to_owned());
        assert_eq!(result, EXPECTED_RESULTS[0]);
    }

    #[test]
    fn test_two_gold() {
        let result = solution::longest_palindrome_gold(PARAMETER_1[1].to_owned());
        let other_valid_answer = "aba";
        if result == EXPECTED_RESULTS[1] {
            assert_eq!(result, EXPECTED_RESULTS[1]);
        } else {
            assert_eq!(result, other_valid_answer);
        }
    }

    #[test]
    fn test_three_gold() {
        let result = solution::longest_palindrome_gold(PARAMETER_1[2].to_owned());
        assert_eq!(result, EXPECTED_RESULTS[2]);
    }

    #[test]
    fn test_four_gold() {
        let result = solution::longest_palindrome_gold(PARAMETER_1[3].to_owned());
        assert_eq!(result, EXPECTED_RESULTS[3]);
    }

    #[test]
    fn test_five_gold() {
        let result = solution::longest_palindrome_gold(PARAMETER_1[4].to_owned());
        assert_eq!(result, EXPECTED_RESULTS[4]);
    }

    #[test]
    fn test_six_gold() {
        let result = solution::longest_palindrome_gold(PARAMETER_1[5].to_owned());
        assert_eq!(result, EXPECTED_RESULTS[5]);
    }
}
