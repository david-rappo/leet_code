#[allow(dead_code)]
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let first_string = strs[0].as_bytes();
    for (index, character) in first_string.iter().enumerate() {
        for s in strs.iter().skip(1) {
            let bytes = s.as_bytes();
            if ((index < bytes.len()) && (bytes[index] != *character)) ||
                (index >= bytes.len()) {
                let prefix = &first_string[..index];
                return String::from_utf8_lossy(prefix).to_string();
            }
        }
    }

    strs[0].to_string()
}