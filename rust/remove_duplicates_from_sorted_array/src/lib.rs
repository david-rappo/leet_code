mod solution;

#[cfg(test)]
mod tests {
    use crate::solution;

    const PARAMETER_1: [&[i32]; 9] = [
        &[0, 1, 2, 2, 2, 3, 4, 5],
        &[1, 2, 3],
        &[],
        &[1],
        &[6, 6, 6],
        &[-1, -1, 1, 2, 3],
        &[10, 11, 12, 13, 13, 13, 13],
        &[10, 10, 11, 12, 13, 13, 13, 13, 14],
        &[1, 1, 1, 1, 1, 2, 3, 4, 5, 5, 5, 6, 7, 8, 8, 9, 10, 11, 11],
    ];
    const EXPECTED_RESULT: [&[i32]; 9] = [
        &[0, 1, 2, 3, 4, 5],
        &[1, 2, 3],
        &[],
        &[1],
        &[6],
        &[-1, 1, 2, 3],
        &[10, 11, 12, 13],
        &[10, 11, 12, 13, 14],
        &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
    ];

    #[test]
    fn test_solution() {
        for (index, parameter_1) in PARAMETER_1.iter().enumerate() {
            let mut copy_parameter_1 = parameter_1.to_vec();
            let result = solution::remove_duplicates(&mut copy_parameter_1);
            let expected_result = EXPECTED_RESULT[index];
            assert!(result > -1);
            let length = result as usize;
            assert_eq!(expected_result.len(), length);
            let slice = &copy_parameter_1[0..length];
            assert_eq!(slice, expected_result);
        }
    }
}
