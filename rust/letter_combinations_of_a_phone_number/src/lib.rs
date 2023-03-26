mod solution;

#[cfg(test)]
mod tests {
    use crate::solution;

    const PARAMETER_0: [&str; 1] = ["23"];
    const EXPECTED_RESULT: [&[&str]; 1] = [&["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]];

    #[test]
    fn test_solution() {
        for pair in PARAMETER_0.iter().enumerate() {
            let parameter_0 = *pair.1;
            let result = solution::letter_combinations(parameter_0.to_owned());
            let expected_result = EXPECTED_RESULT[pair.0];
            // TODO:
            // assert_eq!(4, 4);
        }
    }
}
