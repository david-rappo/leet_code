const BLANK: u8 = b' ';
const MINUS: u8 = b'-';
const PLUS: u8 = b'+';
const DIGITS: [u8; 10] = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];

pub fn string_to_integer(s: String) -> i32 {
    let bytes = s.as_bytes();
    let begin_index = bytes.iter().position(|c| *c != BLANK);
    if begin_index.is_none() {
        return 0;
    }

    let mut begin_index = begin_index.unwrap();
    let is_sign_character = is_sign(bytes[begin_index]);
    let mut is_positive = true;
    if is_sign_character {
        is_positive = PLUS == bytes[begin_index];
        begin_index += 1;
    }
    
    let end_index = bytes.iter().skip(begin_index).position(|c| !is_digit(*c));
    let end_index = match end_index {
        Some(end_index) => end_index,
        None => bytes.len()
    };

    let mut new_bytes = vec![];
    let count = end_index - begin_index;
    if is_sign_character && (!is_positive) {
        new_bytes.reserve(count + 1);
        new_bytes.push(MINUS);
    } else {
        new_bytes.reserve(count);
    }

    for c in bytes.iter().take(end_index).skip(begin_index) {
        new_bytes.push(*c);
    }

    let string_digits = match String::from_utf8(new_bytes) {
        Ok(string_digits) => string_digits,
        Err(_) => String::new()
    };

    let result_32 = string_digits.parse::<i32>();
    if let Ok(result) = result_32 {
        return result;
    }

    match is_positive {
        true => return i32::MAX,
        false => return i32::MIN
    }
}

fn is_sign(c: u8) -> bool {
    matches!(c, MINUS|PLUS)
}

fn is_digit(search_character: u8) -> bool {
    let result = DIGITS.iter().find(|digit| **digit == search_character);
    result.is_some()
}