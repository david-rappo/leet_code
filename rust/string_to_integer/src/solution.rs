const BLANK: u8 = b' ';
const MINUS: u8 = b'-';
const PLUS: u8 = b'+';
const DIGITS: [u8; 10] = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];

#[allow(dead_code)]
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

    if begin_index == bytes.len() {
        return 0;
    }

    let begin_index = first_non_zero_digit_index(bytes, begin_index);
    let begin_index = match begin_index {
        Some(begin_index) => begin_index,
        None => bytes.len()
    };

    if begin_index == bytes.len() {
        return 0;
    }
    
    let end_index = first_non_digit_index(bytes, begin_index);
    let end_index = match end_index {
        Some(end_index) => end_index,
        None => bytes.len()
    };

    let digit_count = end_index - begin_index;
    if digit_count == 0 {
        return 0;
    }
    
    let mut exponent = digit_count - 1;
    let mut result: i32 = 0;
    for digit in bytes.iter().take(end_index).skip(begin_index) {
        let power_result = 10i32.overflowing_pow(exponent as u32);
        if power_result.1 {
            return out_of_range(is_positive);
        }
        
        let digit = digit_to_integer(*digit).unwrap() as i32;
        let product_result = power_result.0.overflowing_mul(digit);
        if product_result.1 {
            return out_of_range(is_positive);
        }

        let addition_result = result.overflowing_add(product_result.0);
        if addition_result.1 {
            return out_of_range(is_positive);
        }

        result = addition_result.0;
        if exponent > 0 {
            exponent -= 1;
        }
    }

    if !is_positive {
        let product_result = result.overflowing_mul(-1);
        if product_result.1 {
            return out_of_range(is_positive);
        }

        result = product_result.0;
    }

    result
}

fn is_sign(c: u8) -> bool {
    matches!(c, MINUS|PLUS)
}

fn is_digit(search_character: u8) -> bool {
    let result = DIGITS.iter().find(|digit| **digit == search_character);
    result.is_some()
}

fn digit_to_integer(digit: u8) -> Option<usize> {
    DIGITS.iter().position(|c| *c == digit)
}

fn out_of_range(is_positive: bool) -> i32 {
    if is_positive {
        i32::MAX
    } else {
        i32::MIN
    }
}

fn first_non_digit_index(bytes: &[u8], begin_index: usize) -> Option<usize> {
    for p in bytes.iter().skip(begin_index).enumerate() {
        if !is_digit(*p.1) {
            // enumerate always starts at zero.
            return Some(p.0 + begin_index);
        }
    }
    
    None
}

fn first_non_zero_digit_index(bytes: &[u8], begin_index: usize) -> Option<usize> {
    for p in bytes.iter().skip(begin_index).enumerate() {
        let c = *p.1;
        if is_digit(c) {
            let zero_digit = DIGITS[0];
            if zero_digit != c {
                // enumerate always starts at zero.
                return Some(p.0 + begin_index);
            }
        } else {
            return None
        }
    }
    
    None
}