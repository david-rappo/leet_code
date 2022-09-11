mod solution;

#[cfg(test)]
mod tests {
    use crate::solution;

    const PARAMETER_1: [&str; 3] = ["III", "LVIII", "MCMXCIV"];
    const EXPECTED_RESULT: [i32; 3] = [3, 58, 1994];

    #[test]
    fn test_solution() {
        for index in 0..PARAMETER_1.len() {
            let result = solution::roman_to_int(PARAMETER_1[index].to_owned());
            assert_eq!(result, EXPECTED_RESULT[index]);
        }
    }
}
