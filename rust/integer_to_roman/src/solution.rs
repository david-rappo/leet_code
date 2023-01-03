use std::collections::HashMap;

const ROMAN_DIVISORS: [i32; 7] = [1000, 500, 100, 50, 10, 5, 1];
const ROMAN_SYMBOLS: [u8; 7] = [b'M', b'D', b'C', b'L', b'X', b'V', b'I'];
const SPECIAL_CASE_4: [u8; 2] = [ROMAN_SYMBOLS[6], ROMAN_SYMBOLS[5]];
const SPECIAL_CASE_9: [u8; 2] = [ROMAN_SYMBOLS[6], ROMAN_SYMBOLS[4]];
const SPECIAL_CASE_40: [u8; 2] = [ROMAN_SYMBOLS[4], ROMAN_SYMBOLS[3]];
const SPECIAL_CASE_90: [u8; 2] = [ROMAN_SYMBOLS[4], ROMAN_SYMBOLS[2]];
const SPECIAL_CASE_400: [u8; 2] = [ROMAN_SYMBOLS[2], ROMAN_SYMBOLS[1]];
const SPECIAL_CASE_900: [u8; 2] = [ROMAN_SYMBOLS[2], ROMAN_SYMBOLS[0]];

#[allow(dead_code)]
pub fn int_to_roman(num: i32) -> String {
    let mut result: Vec<u8> = vec![];
    let coefficients = split(num);
    let special_cases = create_special_cases();
    let mut index = 0;
    debug_assert_eq!(coefficients.len(), ROMAN_DIVISORS.len());
    while index < ROMAN_DIVISORS.len() {
        let remainder = index % 2;
        let previous_index = index;
        if 0 != remainder {
            let x = coefficients[index] * ROMAN_DIVISORS[index];
            let y = coefficients[index + 1] * ROMAN_DIVISORS[index + 1];
            let sum = x + y;
            if let Some(symbols) = special_cases.get(&sum) {
                result.extend_from_slice(symbols);
                index += 2;
            }
        }

        if previous_index == index {
            let product = coefficients[index] * ROMAN_DIVISORS[index];
            if let Some(symbols) = special_cases.get(&product) {
                result.extend_from_slice(symbols);
            } else {
                append(ROMAN_SYMBOLS[index], coefficients[index], &mut result);
            }

            index += 1;
        }
    }

    String::from_utf8(result).unwrap()
}

fn append(symbol: u8, count: i32, result: &mut Vec<u8>) {
    let mut v = vec![symbol; count as usize];
    result.append(&mut v);
}

fn split(mut x: i32) -> Vec<i32> {
    let mut coefficients = Vec::new();
    coefficients.reserve(ROMAN_DIVISORS.len());
    for divisor in ROMAN_DIVISORS.iter() {
        let quotient = x / divisor;
        coefficients.push(quotient);
        x -= quotient * divisor;
    }

    coefficients
}

fn create_special_cases() -> HashMap<i32, &'static [u8]> {
    let mut hash_map: HashMap<i32, &'static [u8]> = HashMap::new();
    hash_map.insert(4, &SPECIAL_CASE_4);
    hash_map.insert(9, &SPECIAL_CASE_9);
    hash_map.insert(40, &SPECIAL_CASE_40);
    hash_map.insert(90, &SPECIAL_CASE_90);
    hash_map.insert(400, &SPECIAL_CASE_400);
    hash_map.insert(900, &SPECIAL_CASE_900);
    hash_map
}
