#[allow(dead_code)]
pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows < 1 {
        return String::new();
    }

    // For example,
    // s = "coffee"
    // num_rows = 20
    //
    // Six rows at most are required.
    let row_count = usize::min(num_rows as usize, s.len());
    let bytes = s.as_bytes();
    let mut strings = vec![vec![]; row_count];
    let mut row_index = 0;
    let mut is_vertical_traverse = true;
    let last_row_index = row_count - 1;
    let second_row_from_top = 1;
    for character in bytes {
        strings[row_index].push(*character);
        if is_vertical_traverse {
            if last_row_index == row_index {
                row_index = last_row_index.saturating_sub(1);
                // If the result of saturating_sub is zero, then there is only
                // one row. Therefore, the value of is_vertical_traverse should
                // be left as true.
                if 0 != row_index {
                    is_vertical_traverse = false;
                }
            } else {
                row_index += 1;
            }
        } else {
            let previous_row_index = row_index;
            row_index -= 1;
            if second_row_from_top == previous_row_index {
                is_vertical_traverse = true;
            }
        }
    }

    create_result(&strings, s.len())
}

fn create_result(strings: &[Vec<u8>], length: usize) -> String {
    let mut result = Vec::with_capacity(length);
    for row in strings.iter() {
        for character in row {
            result.push(*character);
        }
    }

    String::from_utf8(result).unwrap()
}