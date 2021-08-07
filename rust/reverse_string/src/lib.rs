mod jade;
mod solution;

#[cfg(test)]
mod tests {
    const TEST_1_VECTOR: [char; 6] = ['C', 'o', 'f', 'f', 'e', 'e'];
    const TEST_1_EXPECTED_RESULT: [char; 6] = ['e', 'e', 'f', 'f', 'o', 'C'];

    const TEST_2_VECTOR: [char; 0] = [];
    const TEST_2_EXPECTED_RESULT: [char; 0] = [];

    const TEST_3_VECTOR: [char; 1] = ['x'];
    const TEST_3_EXPECTED_RESULT: [char; 1] = ['x'];

    const TEST_4_VECTOR: [char; 3] = ['r', 'e', 'd'];
    const TEST_4_EXPECTED_RESULT: [char; 3] = ['d', 'e', 'r'];

    #[test]
    fn test_1() {
        use crate::solution;

        let mut v = TEST_1_VECTOR.to_vec();
        solution::reverse_string(&mut v);
        assert_eq!(v, TEST_1_EXPECTED_RESULT);
    }

    #[test]
    fn test_2() {
        use crate::solution;

        let mut v = TEST_2_VECTOR.to_vec();
        solution::reverse_string(&mut v);
        assert_eq!(v, TEST_2_EXPECTED_RESULT);
    }

    #[test]
    fn test_3() {
        use crate::solution;

        let mut v = TEST_3_VECTOR.to_vec();
        solution::reverse_string(&mut v);
        assert_eq!(v, TEST_3_EXPECTED_RESULT);
    }

    #[test]
    fn test_4() {
        use crate::solution;

        let mut v = TEST_4_VECTOR.to_vec();
        solution::reverse_string(&mut v);
        assert_eq!(v, TEST_4_EXPECTED_RESULT);
    }
}