mod solution;

#[cfg(test)]
mod tests {
    use crate::solution;

    const PARAMETER_1: [&str; 3] = ["()", "()[]{}", "(]"];
    const EXPECTED_RESULT: [bool; 3] = [true, true, false];

    #[test]
    fn test() {
        for (index, element) in PARAMETER_1.iter().enumerate() {
            let result = solution::is_valid(element.to_string());
            assert_eq!(result, EXPECTED_RESULT[index]);
        }
    }
}