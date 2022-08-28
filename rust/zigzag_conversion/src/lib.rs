mod leet_code;
mod solution;

#[cfg(test)]
mod tests {
    use crate::leet_code;
    use crate::solution;

    const PARAMETER_1: [&str; 4] = ["PAYPALISHIRING",
        "PAYPALISHIRING",
        "PAYPALISHIRING",
        "PAYPALISHIRING"];
    const PARAMETER_2: [usize; 4] = [3, 4, 1, 2];
    const EXPECTED_RESULT: [&str; 4] = ["PAHNAPLSIIGYIR",
        "PINALSIGYAHRPI",
        "PAYPALISHIRING",
        "PYAIHRNAPLSIIG"];

    #[test]
    fn test_solution() {
        for index in 0..PARAMETER_1.len() {
            let result = solution::convert(PARAMETER_1[index].to_owned(),
                PARAMETER_2[index] as i32);
            assert_eq!(result, EXPECTED_RESULT[index]);
        }
    }

    #[test]
    fn test_leet_code() {
        for index in 0..PARAMETER_1.len() {
            let result = leet_code::convert(PARAMETER_1[index].to_owned(),
                PARAMETER_2[index] as i32);
            assert_eq!(result, EXPECTED_RESULT[index]);
        }
    }
}
