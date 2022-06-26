#[allow(dead_code)]

pub fn longest_palindrome_gold(s: String) -> String {
    if s.is_empty() {
        return String::new();
    }
    
    let bytes = s.as_bytes();
    // best_range is inclusive. For example, [begin_index, last_index]
    let mut best_range: (usize, usize) = (0, 0);
    for index in 0..bytes.len() {
        let mut new_range = (index, index);
        if is_even_palindrome_two(bytes, index) {
            new_range = (index, index + 1);
        } else if is_odd_palindrome_three(bytes, index) {
            new_range = (index - 1, index + 1);
        }

        if new_range.0 < new_range.1 {
            new_range = get_palindrome_greater_one(bytes, &new_range);
        }

        let best_range_length = get_length(&best_range);
        let new_range_length = get_length(&new_range);
        if new_range_length > best_range_length {
            best_range = new_range;
        }
    }
    
    create_string(bytes, &best_range)
}

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
// So is the result range returned by get_palindrome.
fn get_palindrome_greater_one(bytes: &[u8], range: &(usize, usize)) -> (usize, usize) {
    debug_assert!(range.1 < bytes.len());
    debug_assert!(range.0 < range.1);
    let mut i = range.0;
    let mut j = range.1;
    let mut result_range = (i, j);
    while (j < bytes.len()) && (bytes[i] == bytes[j]) {
        result_range = (i, j);
        // Since i has type usize it is necessary to check if i is zero before
        // decrementing it.
        if i == 0 {
            break;
        }

        i -= 1;
        j += 1;
    }
    
    result_range
}

// For example, bytes = "abaxyzzyxf"
//              index = 5
fn is_even_palindrome_two(bytes: &[u8], index: usize) -> bool {
    debug_assert!(index < bytes.len());
    let character = bytes[index];
    let right_index = index + 1;
    if right_index < bytes.len() {
        let right_character = bytes[right_index];
        if character == right_character {
            return true;
        }
    }

    false
}

fn is_odd_palindrome_three(bytes: &[u8], index: usize) -> bool {
    debug_assert!(index < bytes.len());
    let right_index = index + 1;
    if (index > 0) && (right_index < bytes.len()) {
        let left_index = index - 1;
        let left_character = bytes[left_index];
        let right_character = bytes[right_index];
        return left_character == right_character;
    }
    
    false
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