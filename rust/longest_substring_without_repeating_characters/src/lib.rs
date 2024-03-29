mod solution;

#[cfg(test)]
mod tests {
    use crate::solution;

    const ARGUMENTS: [&str; 4] = ["abcabcbb", "bbbbb", "pwwkew", "dvdf"];
    const EXPECTED_RESULTS: [usize; 4] = [3, 1, 3, 3];

    #[test]
    fn all_tests() {
        for index in 0..ARGUMENTS.len() {
            let result = solution::length_of_longest_substring(ARGUMENTS[index]);
            assert_eq!(result, EXPECTED_RESULTS[index]);
        }
    }
}