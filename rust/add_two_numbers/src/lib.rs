mod solution;

#[cfg(test)]
mod tests {
    use crate::solution;

    fn is_equal(expected_result: &[i32],
        linked_list: &Option<Box<solution::ListNode>>) -> bool {
        let mut current = linked_list;
        for number in expected_result {
            match current.as_ref() {
                Some(match_current) => {
                    if match_current.val != *number {
                        return false;
                    }
                },
                None => {
                    return false;
                }
            }

            current = match current.as_ref() {
                Some(match_current) => {
                    &match_current.next
                },
                _ => unreachable!()
            }
        }
        
        true
    }

    #[test]
    fn example_one() {
        let x = solution::create_list(&[2, 4, 3]);
        let y = solution::create_list(&[5, 6, 4]);
        let expected_result = [7, 0, 8];
        let result = solution::add_two_numbers(x, y);
        assert_eq!(true, is_equal(&expected_result, &result));
    }

    #[test]
    fn example_two() {
        let x = solution::create_list(&[0]);
        let y = solution::create_list(&[0]);
        let expected_result = [0];
        let result = solution::add_two_numbers(x, y);
        assert_eq!(true, is_equal(&expected_result, &result));
    }

    #[test]
    fn example_three() {
        let x = solution::create_list(&[7, 7, 7, 7, 7, 7, 7]);
        let y = solution::create_list(&[9, 9, 9, 9]);
        let expected_result = [8, 9, 9, 9, 0, 0, 0, 1];
        let result = solution::add_two_numbers(x, y);
        assert_eq!(true, is_equal(&expected_result, &result));
    }
}