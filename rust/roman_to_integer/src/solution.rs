use std::collections::HashMap;

// TODO: Untested - check.
#[allow(dead_code)]
pub fn roman_to_int(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }
    
    let bytes = s.as_bytes();
    let roman_numeral_to_value = create_roman_numeral_to_value();
    if bytes.len() < 2 {
        return *roman_numeral_to_value.get(&bytes[0]).unwrap();
    }

    let mut index = 0;
    let mut sum = 0;
    while index < bytes.len() - 1 {
        let character = bytes[index];
        let value = roman_numeral_to_value.get(&character).unwrap();
        let next_character = bytes[index + 1];
        let next_value = roman_numeral_to_value.get(&next_character).unwrap();
        // For example, value = I
        //              next_value = V
        if next_value > value {
            let difference = next_value - value;
            sum += difference;
            index += 2;
        } else {
            sum += value;
            index += 1;
        }
    }

    if index < bytes.len() {
        let character = bytes[index];
        let value = roman_numeral_to_value.get(&character).unwrap();
        sum += value;
    }
    
    sum
}

fn create_roman_numeral_to_value() -> HashMap<u8, i32> {
    let mut hash_map = HashMap::new();
    hash_map.insert(b'I', 1);
    hash_map.insert(b'V', 5);
    hash_map.insert(b'X', 10);
    hash_map.insert(b'L', 50);
    hash_map.insert(b'C', 100);
    hash_map.insert(b'D', 500);
    hash_map.insert(b'M', 1000);
    hash_map
}