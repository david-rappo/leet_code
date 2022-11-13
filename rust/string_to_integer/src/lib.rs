mod solution;

#[cfg(test)]
mod tests {
    use crate::solution;

    const PARAMETER_1: [&str; 15] = ["4294967296.50", // This should be clamped to i32::MAX
        "-42.0",
        "42",
        "-42",
        "4193 with words",
        "   100",
        "   +100",
        "   -100",
        "77 ",
        "77?",
        "77##",
        "2147483647",
        "+2147483648", // This should be clamped to i32::MAX
        "-2147483648",
        " -2147483649 "]; // This should be clamped to i32::MIN
    const EXPECTED_RESULT: [i32; 15] = [i32::MAX,
        -42,
        42,
        -42,
        4193,
        100,
        100,
        -100,
        77,
        77,
        77,
        i32::MAX,
        i32::MAX,
        i32::MIN,
        i32::MIN];

    #[test]
    fn test_string_to_integer() {
        for pair in PARAMETER_1.iter().enumerate() {
            let slice = *pair.1;
            let result = solution::string_to_integer(slice.to_owned());
            assert_eq!(result, EXPECTED_RESULT[pair.0]);
        }
    }
}
