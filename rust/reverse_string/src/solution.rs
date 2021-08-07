// Module: solution

#[allow(dead_code)]
pub fn reverse_string(s: &mut Vec<char>) {
    // For example,
    // Initial: abcd
    // Pass 0   dXXa
    // Pass 1   dcba
    if s.is_empty() {
        return;
    }

    let mut index_i = 0;
    let mut index_j = s.len() - 1;
    while index_i < index_j {
        s.swap(index_i, index_j);
        index_i += 1;
        index_j -= 1;
    }
}