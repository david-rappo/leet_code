use std::collections::HashMap;

const OPEN_PARENTHESES: &[u8] = "({[".as_bytes();
const CLOSE_PARENTHESES: &[u8] = ")}]".as_bytes();

#[allow(dead_code)]
pub fn is_valid(s: String) -> bool {
    let bytes = s.as_bytes();
    let mut stack_open_parentheses = vec![0; 0];
    let open_to_close_parentheses = create_open_to_close_parentheses();
    for character in bytes.iter() {
        if open_to_close_parentheses.contains_key(character) {
            stack_open_parentheses.push(*character);
        } else if is_close_parentheses(*character) {
            if stack_open_parentheses.is_empty() {
                return false;
            }

            let last_character = stack_open_parentheses.pop().unwrap();
            let expected_character = open_to_close_parentheses.get(&last_character).unwrap();
            if expected_character != character {
                return false;
            }
        }
    }
    
    stack_open_parentheses.is_empty()
}

fn is_close_parentheses(parentheses: u8) -> bool {
    CLOSE_PARENTHESES.iter().any(|x| *x == parentheses)
}

fn create_open_to_close_parentheses() -> HashMap<u8, u8> {
    let mut hash_map = HashMap::new();
    for index in 0..OPEN_PARENTHESES.len() {
        hash_map.insert(OPEN_PARENTHESES[index], CLOSE_PARENTHESES[index]);
    }

    hash_map
}