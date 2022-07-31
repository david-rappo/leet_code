#[allow(dead_code)]
pub fn is_palindrome(x: i32) -> bool {
    let x_string = format!("{}", x);
    if x_string.is_empty() {
        return false;
    }
    
    let bytes = x_string.as_bytes();
    let mut i = 0;
    let mut j = bytes.len() - 1;
    while i < j {
        if bytes[i] != bytes[j] {
            return false;
        }

        i += 1;
        j -= 1;
    }
    
    true
}