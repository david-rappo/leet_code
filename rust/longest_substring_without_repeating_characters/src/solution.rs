use std::collections::HashMap;

#[allow(dead_code)]
pub fn length_of_longest_substring(s: &str) -> usize {
    if s.is_empty() {
        return 0;
    }

    let mut hash_map = HashMap::new();
    let bytes = s.as_bytes();
    let mut result_range = (0, 0);
    let mut begin_index = 0;
    for index in 0..bytes.len() {
        let character = bytes[index];
        if hash_map.contains_key(&character) {
            let new_result_range = (begin_index, index);
            if is_greater(&new_result_range, &result_range) {
                result_range = new_result_range;
            }

            // For example,
            // 0 1 2 3 4 5 6 7
            // a b c a b c b b
            //       B     I
            //           X
            let index_of_character = hash_map.get(&character);
            debug_assert!(index_of_character.is_some());
            let previous_begin_index = begin_index;
            begin_index = index_of_character.unwrap() + 1;
            remove(previous_begin_index, begin_index, bytes, &mut hash_map);
            hash_map.insert(character, index);
        } else {
            hash_map.insert(character, index);
        }
    }
    
    if !hash_map.is_empty() {
        let new_result_range = (begin_index, bytes.len());
        if is_greater(&new_result_range, &result_range) {
            result_range = new_result_range;
        }
    }
    
    result_range.1 - result_range.0
}

fn remove(begin_index: usize,
    end_index: usize,
    bytes: &[u8],
    hash_map: &mut HashMap<u8, usize>) {
    for iter in bytes.iter().take(end_index).skip(begin_index) {
        hash_map.remove(iter);
    }
}

fn get_length(range: &(usize, usize)) -> usize {
    range.1 - range.0
}

fn is_greater(left_range: &(usize, usize), right_range: &(usize, usize)) -> bool {
    let left_length = get_length(left_range);
    let right_length = get_length(right_range);
    left_length > right_length
}