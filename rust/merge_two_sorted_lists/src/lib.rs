mod solution;
mod list_node;

#[cfg(test)]
mod tests {
    use crate::solution;
    use crate::list_node;

    const MERGE_TWO_LISTS_PARAMETER_1: [&[i32]; 6] = [&[1, 2, 4],
        &[],
        &[],
        &[0],
        &[10, 11],
        &[]];
    const MERGE_TWO_LISTS_PARAMETER_2: [&[i32]; 6] = [&[1, 4, 4],
        &[],
        &[0],
        &[],
        &[],
        &[10, 11]];
    const MERGE_TWO_LISTS_EXPECTED_RESULT: [&[i32]; 6] = [&[1, 1, 2, 3, 4, 4],
        &[],
        &[0],
        &[0],
        &[10, 11],
        &[10, 11]];
    const REVERSE_PARAMETER_1: [&[i32]; 4] = [&[1, 2, 3, 4, 5],
        &[],
        &[1],
        &[10, 11]];
    const REVERSE_EXPECTED_RESULT: [&[i32]; 4] = [&[5, 4, 3, 2, 1],
        &[],
        &[1],
        &[11, 10]];
    const POP_FRONT_PARAMETER_1: [&[i32]; 4] = [&[],
        &[1],
        &[1, 2],
        &[1, 2, 3]];
    // Nodes remaining in input list after the first node is popped.
    const POP_FRONT_EXPECTED_RESULT: [&[i32]; 4] = [&[],
        &[],
        &[2],
        &[2, 3]];
    const PUSH_FRONT_PARAMETER_1: [&[i32]; 5] = [&[2, 3, 4],
        &[],
        &[1],
        &[],
        &[2]];
    const PUSH_FRONT_PARAMETER_2: [&[i32]; 5] = [&[1],
        &[],
        &[],
        &[5],
        &[1]];
    const PUSH_FRONT_EXPECTED_RESULT: [&[i32]; 5] = [&[1, 2, 3, 4],
        &[],
        &[1],
        &[5],
        &[1, 2]];

    fn create_list(mut numbers: Vec<i32>) -> Option<Box<list_node::ListNode>> {
        numbers.reverse();
        let mut result = None;
        for number in numbers {
            let node = list_node::ListNode { val: number, next: None };
            result = solution::push_front(result, Some(Box::new(node)));
        }

        result
    }

    fn get_list_values(list: &Option<Box<list_node::ListNode>>) -> Vec<i32> {
        let mut result = vec![];
        let mut list = list;
        while list.is_some() {
            match list {
                Some(boxed_list) => {
                    result.push(boxed_list.val);
                    list = &boxed_list.next;
                },
                None => {
                    return result;
                }
            }
        }
        
        result
    }

    #[test]
    fn test_merge_two_lists() {
        assert_eq!(MERGE_TWO_LISTS_PARAMETER_1.len(), MERGE_TWO_LISTS_PARAMETER_2.len());
        assert_eq!(MERGE_TWO_LISTS_PARAMETER_2.len(), MERGE_TWO_LISTS_EXPECTED_RESULT.len());
        for index in 0..MERGE_TWO_LISTS_PARAMETER_1.len() {
            let parameter_1 = MERGE_TWO_LISTS_PARAMETER_1[index];
            let parameter_2 = MERGE_TWO_LISTS_PARAMETER_2[index];
            let list_1 = create_list(parameter_1.to_vec());
            let list_2 = create_list(parameter_2.to_vec());
            let result = solution::merge_two_lists(list_1, list_2);
            let result = get_list_values(&result);
            let expected_result = MERGE_TWO_LISTS_EXPECTED_RESULT[index];
            assert_eq!(result, expected_result);
        }
    }

    #[test]
    fn test_reverse() {
        assert_eq!(REVERSE_PARAMETER_1.len(), REVERSE_EXPECTED_RESULT.len());
        for index in 0..REVERSE_PARAMETER_1.len() {
            let parameter_1 = REVERSE_PARAMETER_1[index];
            let list = create_list(parameter_1.to_vec());
            let result = solution::reverse(list);
            let result = get_list_values(&result);
            let expected_result = REVERSE_EXPECTED_RESULT[index];
            assert_eq!(result, expected_result);
        }
    }

    #[test]
    fn test_pop_front() {
        assert_eq!(POP_FRONT_PARAMETER_1.len(), POP_FRONT_EXPECTED_RESULT.len());
        for index in 0..POP_FRONT_PARAMETER_1.len() {
            let parameter_1 = POP_FRONT_PARAMETER_1[index];
            let list = create_list(parameter_1.to_vec());
            let pair = solution::pop_front(list);
            let result = get_list_values(&pair.0);
            let expected_result = POP_FRONT_EXPECTED_RESULT[index];
            assert_eq!(result, expected_result);
            if !expected_result.is_empty() {
                let popped_value = get_list_values(&pair.1)[0];
                let first_value = parameter_1[0];
                assert_eq!(popped_value, first_value);
            }
        }
    }

    #[test]
    fn test_push_front() {
        assert_eq!(PUSH_FRONT_PARAMETER_1.len(), PUSH_FRONT_PARAMETER_2.len());
        assert_eq!(PUSH_FRONT_PARAMETER_2.len(), PUSH_FRONT_EXPECTED_RESULT.len());
        for index in 0..PUSH_FRONT_PARAMETER_1.len() {
            let parameter_1 = PUSH_FRONT_PARAMETER_1[index];
            let parameter_2 = PUSH_FRONT_PARAMETER_2[index];
            let list = create_list(parameter_1.to_vec());
            let node = create_list(parameter_2.to_vec());
            let result = solution::push_front(list, node);
            let result = get_list_values(&result);
            let expected_result = PUSH_FRONT_EXPECTED_RESULT[index];
            assert_eq!(result, expected_result);
        }
    }
}