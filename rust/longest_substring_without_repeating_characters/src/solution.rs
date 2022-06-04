use std::collections::HashSet;

pub fn length_of_longest_substring(s: &str) -> usize {
    if s.is_empty() {
        return 0;
    }

    let mut hash_set = HashSet::new();
    let bytes = s.as_bytes();
    let mut result_range = (0, 0);
    // For example,
    // s = abccxy
    // substring 0: [0, 3)
    // substring 0: [3, 6)
    for index in 0..bytes.len() {
        let character = bytes[index];
        if !hash_set.insert(character) {
            result_range = (result_range.1, index);
            hash_set.clear();
            hash_set.insert(character);
        }
    }

    if !hash_set.is_empty() {
        result_range = (result_range.1, s.len());
    }

    result_range.1 - result_range.0
}