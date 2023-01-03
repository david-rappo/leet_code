mod solution;

#[cfg(test)]
mod tests {
    use crate::solution::int_to_roman;

    const PARAMETER_1: [i32; 22] = [
        3, 58, 1994, 4, 9, 40, 90, 400, 900, 5, 10, 50, 100, 500, 1000, 104, 1009, 550, 540, 390,
        444, 999,
    ];
    const EXPECTED_RESULT: [&str; 22] = [
        "III", "LVIII", "MCMXCIV", "IV", "IX", "XL", "XC", "CD", "CM", "V", "X", "L", "C", "D",
        "M", "CIV", "MIX", "DL", "DXL", "CCCXC", "CDXLIV", "CMXCIX",
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
