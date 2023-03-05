mod solution;

#[cfg(test)]
mod tests {
    use crate::solution;

    const PARAMETER_1: [&[i32]; 1] = [&[0, 1, 2, 2, 2, 3, 4, 5]];
    const EXPECTED_RESULT_COUNT: [i32; 1] = [7];
    const EXPECTED_RESULT: [&[i32]; 1] = [&[0, 1, 2, 3, 4, 5]];

    #[test]
    fn test_solution() {
        for (index, parameter_1) in PARAMETER_1.iter().enumerate() {
            let mut copy_parameter_1 = parameter_1.to_vec();
            let result = solution::remove_duplicates(&mut copy_parameter_1);
            assert_eq!(EXPECTED_RESULT_COUNT[index], result);
            let length = result as usize;
            let slice = &copy_parameter_1[0..length];
            assert_eq!(slice, EXPECTED_RESULT[index]);
        }
    }
}
