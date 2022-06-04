use std::collections::HashSet;

#[allow(dead_code)]
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
    let mut begin_index = 0;
    for index in 0..bytes.len() {
        let character = bytes[index];
        if !hash_set.insert(character) {
            let new_result_range = (begin_index, index);
            if is_greater(&new_result_range, &result_range) {
                result_range = new_result_range;
            }
            
            begin_index = index;
            hash_set.clear();
            hash_set.insert(character);
        }
    }

    if !hash_set.is_empty() {
        let new_result_range = (begin_index, bytes.len());
        if is_greater(&new_result_range, &result_range) {
            result_range = new_result_range;
        }
    }

    result_range.1 - result_range.0
}

fn get_length(range: &(usize, usize)) -> usize {
    range.1 - range.0
}

fn is_greater(left_range: &(usize, usize), right_range: &(usize, usize)) -> bool {
    let left_length = get_length(left_range);
    let right_length = get_length(right_range);
    left_length > right_length
}