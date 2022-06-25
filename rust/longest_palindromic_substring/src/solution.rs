#[allow(dead_code)]
pub fn longest_palindrome(s: String) -> String {
    if s.is_empty() {
        return String::new();
    }

    let bytes = s.as_bytes();
    // best_range is inclusive. For example, [begin_index, last_index]
    let mut best_range: (usize, usize) = (0, 0);
    for begin_index in 0..bytes.len() {
        for last_index in begin_index + 1..bytes.len() {
            let new_range = (begin_index, last_index);
            if is_palindrome(bytes, &new_range) && (get_length(&new_range) > get_length(&best_range)) {
                best_range = new_range;
            }
        }
    }
    
    create_string(bytes, &best_range)
}

// range is inclusive. For example, [begin_index, last_index]
fn get_length(range: &(usize, usize)) -> usize {
    range.1 - range.0 + 1
}

// Range = [begin_index, last_index]
fn is_palindrome(bytes: &[u8], range: &(usize, usize)) -> bool {
    let mut i = range.0;
    let mut j = range.1;
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

// range is inclusive. For example, [begin_index, last_index]
fn create_string(bytes: &[u8], range: &(usize, usize)) -> String {
    let v = Vec::from(&bytes[range.0..range.1 + 1]);
    String::from_utf8(v).unwrap()
}