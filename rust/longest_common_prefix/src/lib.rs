mod solution;

#[cfg(test)]
mod tests {
    use crate::solution;

    const PARAMETER_1: [&[&str]; 4] = [&["flower", "flow", "flight"],
        &["dog", "racecar", "car"],
        &["a"],
        &["coffee", "coffe", "coffe"]];
    const EXPECTED_RESULT: [&str; 4] = ["fl", "", "a", "coffe"];

    fn create_vec(index: usize) -> Vec<String> {
        let mut v = vec![];
        let slice = PARAMETER_1[index];
        v.reserve(slice.len());
        for s in slice {
            v.push(s.to_string());
        }

        v
    }

    #[test]
    fn test_solution() {
        for (index, _) in PARAMETER_1.iter().enumerate() {
            let result = solution::longest_common_prefix(create_vec(index));
            assert_eq!(result, EXPECTED_RESULT[index]);
        }
    }
}