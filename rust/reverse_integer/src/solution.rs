#[allow(dead_code)]
pub fn reverse(x: i32) -> i32 {
    let is_signed = x < 0;
    let str_x = format!("{}", x.unsigned_abs());
    let mut bytes = str_x.into_bytes();
    let mut i = 0;
    let mut j = bytes.len() - 1;
    while i < j {
        bytes.swap(i, j);
        i += 1;
        j -= 1;
    }

    let str_x = String::from_utf8_lossy(&bytes);
    let result: i32 = match str_x.parse() {
        Ok(value) => {
            value
        },
        Err(_) => {
            0
        }
    };
    
    if is_signed {
        -result
    } else {
        result
    }
}