const DIGIT_TO_LETTERS: [&[u8]; 10] = [
    &[],
    &[],
    &[b'a', b'b', b'c'],
    &[b'd', b'e', b'f'],
    &[b'g', b'h', b'i'],
    &[b'j', b'k', b'l'],
    &[b'm', b'n', b'o'],
    &[b'p', b'q', b'r', b's'],
    &[b't', b'u', b'v'],
    &[b'w', b'x', b'y', b'z'],
];

pub fn letter_combinations(digits: String) -> Vec<String> {
    let mut results = vec![];
    let bytes = digits.as_bytes();
    helper(String::new(), 0, bytes, &mut results);
    results
}

fn helper(prefix: String, index: usize, digits: &[u8], results: &mut Vec<String>) {
    if index >= digits.len() {
        results.push(prefix);
        return;
    }

    let digit = digits[index];
    let digit = char::from_u32(digit as u32).unwrap().to_digit(10).unwrap();
    let letters = DIGIT_TO_LETTERS[digit as usize];
    for letter in letters {
        let new_prefix = format!("{}{}", prefix, letter);
        helper(new_prefix, index + 1, digits, results);
    }
}
