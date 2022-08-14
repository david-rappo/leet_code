mod solution;

#[cfg(test)]
mod tests {
    use crate::solution;

    const PARAMETER_1: [&str; 2] = ["PAYPALISHIRING", "PAYPALISHIRING"];
    const PARAMETER_2: [usize; 2] = [3, 4];
    const EXPECTED_RESULT: [&str; 2] = ["PAHNAPLSIIGYIR", "PINALSIGYAHRPI"];

    #[test]
    fn test_solution() {
        for index in 0..PARAMETER_1.len() {
            let result = solution::convert(PARAMETER_1[index].to_owned(),
                PARAMETER_2[index] as i32);
            assert_eq!(result, EXPECTED_RESULT[index]);
        }
    }
}
