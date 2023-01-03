mod solution;

#[cfg(test)]
mod tests {
    use crate::solution::int_to_roman;

    const PARAMETER_1: [i32; 17] = [
        3, 58, 1994, 4, 9, 40, 90, 400, 900, 5, 10, 50, 100, 500, 1000, 104, 1009,
    ];
    const EXPECTED_RESULT: [&str; 17] = [
        "III", "LVIII", "MCMXCIV", "IV", "IX", "XL", "XC", "CD", "CM", "V", "X", "L", "C", "D",
        "M", "CIV", "MIX",
    ];

    #[test]
    fn test_solution() {
        assert_eq!(PARAMETER_1.len(), EXPECTED_RESULT.len());
        for index in 0..PARAMETER_1.len() {
            let parameter_1 = PARAMETER_1[index];
            let expected_result = EXPECTED_RESULT[index];
            let result = int_to_roman(parameter_1);
            assert_eq!(result, expected_result);
        }
    }
}
