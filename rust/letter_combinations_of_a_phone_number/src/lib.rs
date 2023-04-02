mod solution;

#[cfg(test)]
mod tests {
    use crate::solution;

    use std::collections::HashSet;

    const PARAMETER_0: [&str; 3] = ["23", "", "2"];
    const EXPECTED_RESULT: [&[&str]; 3] = [
        &["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"],
        &[],
        &["a", "b", "c"],
    ];

    fn copy_slice(slice: &[&str]) -> Vec<String> {
        let mut result = vec![];
        result.reserve_exact(slice.len());
        for s in slice {
            result.push((*s).to_owned());
        }

        result
    }

    fn create_hash_set(v: Vec<String>) -> HashSet<String> {
        let mut hash_set = HashSet::new();
        for s in v {
            hash_set.insert(s);
        }

        hash_set
    }

    #[test]
    fn test_solution() {
        for pair in PARAMETER_0.iter().enumerate() {
            let parameter_0 = *pair.1;
            let result = solution::letter_combinations(parameter_0.to_owned());
            let expected_result = copy_slice(EXPECTED_RESULT[pair.0]);
            assert_eq!(result.len(), expected_result.len());
            let mut hash_set = create_hash_set(expected_result);
            assert_eq!(result.len(), hash_set.len());
            for s in result.iter() {
                assert!(hash_set.remove(s));
            }

            assert!(hash_set.is_empty());
        }
    }
}
