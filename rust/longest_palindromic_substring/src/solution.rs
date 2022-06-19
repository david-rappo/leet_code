use std::collections::HashMap;

#[allow(dead_code)]
pub fn longest_palindrome(s: String) -> String {
    if s.is_empty() {
        return String::new();
    }

    let bytes = s.as_bytes();
    let hash_map = create_hash_map(bytes);
    let mut range_longest_palindrome: (usize, usize) = (0, 1);
    for iter in hash_map.iter() {
        if iter.1.len() > 1 {
            let begin_index = iter.1[0];
            let last_index = iter.1[iter.1.len() - 1];
            if is_palindrome(bytes, begin_index, last_index) {
                let new_range = (begin_index, last_index + 1);
                if get_length(&new_range) > get_length(&range_longest_palindrome) {
                    range_longest_palindrome = new_range;
                }
            }
        }
    }
    
    create_string(bytes, &range_longest_palindrome)
}

fn get_length(range: &(usize, usize)) -> usize {
    debug_assert!(range.0 < range.1);
    range.1 - range.0
}

// Range = [begin_index, last_index]
fn is_palindrome(bytes: &[u8], begin_index: usize, last_index: usize) -> bool {
    if begin_index == last_index {
        return true;
    }
    
    let mut i = begin_index;
    let mut j = last_index;
    while i < j {
        if bytes[i] != bytes[j] {
            return false;
        }
        
        i += 1;
        if j != 0 {
            j -= 1;
        }

        if j == 0 {
            break;
        }
    }

    true
}

fn create_string(bytes: &[u8], range: &(usize, usize)) -> String {
    let v = Vec::from(&bytes[range.0..range.1]);
    String::from_utf8(v).unwrap()
}

fn create_hash_map(bytes: &[u8]) -> HashMap<u8, Vec<usize>> {
    let mut hash_map: HashMap<u8, Vec<usize>> = HashMap::new();
    for iter in bytes.iter().enumerate() {
        let index = iter.0;
        let b = iter.1;
        if hash_map.contains_key(b) {
            let v = hash_map.get_mut(b);
            v.unwrap().push(index);
        } else {
            hash_map.insert(*b, vec![index; 1]);
        }
    }

    hash_map
}