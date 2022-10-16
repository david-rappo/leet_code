const BLANK: u8 = b' ';
const MINUS: u8 = b'-';
const PLUS: u8 = b'+';
const DIGITS: [u8; 10] = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];

pub fn string_to_integer(s: String) -> i32 {
    let bytes = s.as_bytes();
    let begin_index = find_begin(bytes);
    if begin_index.is_none() {
        return 0;
    }

    let is_sign_character = is_sign(bytes[begin_index.unwrap()]);
    let mut begin_index = begin_index.unwrap();
    let mut is_positive = true;
    if is_sign_character {
        is_positive = PLUS == bytes[begin_index];
        begin_index += 1;
    }
    
    let mut digits: Vec<u8> = vec![];
    for c in bytes.iter().skip(begin_index) {
        if is_digit(*c) {
            digits.push(*c);
        } else {
            break;
        }
    }
    
    let result = String::from_utf8_lossy(&digits);
    let result = result.parse::<i64>();
    match result {
        Ok(mut value) => {
            if !is_positive {
                value *= -1;
            }

            // TODO: Try converting value to i32.
        },
        Err(_) => {
            return 0;
        }
    }
    
    0
}

fn find_begin(bytes: &[u8]) -> Option<usize> {
    for (index, c) in bytes.iter().enumerate() {
        if *c != BLANK {
            return Some(index);
        }
    }
    
    None
}

fn is_sign(c: u8) -> bool {
    matches!(c, MINUS|PLUS)
}

fn is_digit(c: u8) -> bool {
    let result = DIGITS.iter().find(|e| **e == c);
    result.is_some()
}