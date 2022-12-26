use crate::list_node::ListNode;

#[allow(dead_code)]
pub fn merge_two_lists_without_reverse(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut list1 = list1;
    let mut list2 = list2;
    let mut tail: &mut Option<Box<ListNode>> = &mut head;
    while list1.is_some() && list2.is_some() {
        let value_1 = get_some_node_value(&list1);
        let value_2 = get_some_node_value(&list2);
        let pair;
        if value_1 < value_2 {
            pair = pop_front(list1);
            list1 = pair.0;
        } else {
            pair = pop_front(list2);
            list2 = pair.0;
        }

        tail = match tail {
            Some(t) => {
                t.next = pair.1;
                &mut t.next
            }
            None => {
                head = pair.1;
                &mut head
            }
        }
    }

    let mut list = if list1.is_none() { list2 } else { list1 };
    if list.is_none() {
        return head;
    }

    while list.is_some() {
        let pair = pop_front(list);
        list = pair.0;
        tail = match tail {
            Some(t) => {
                t.next = pair.1;
                &mut t.next
            }
            None => {
                head = pair.1;
                &mut head
            }
        }
    }

    head
}

#[allow(dead_code)]
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut result = None;
    let mut list1 = list1;
    let mut list2 = list2;
    loop {
        if list1.is_none() || list2.is_none() {
            break;
        }

        let value_1 = get_some_node_value(&list1);
        let value_2 = get_some_node_value(&list2);
        if value_1 < value_2 {
            let pair = pop_front(list1);
            list1 = pair.0;
            result = push_front(result, pair.1);
        } else {
            let pair = pop_front(list2);
            list2 = pair.0;
            result = push_front(result, pair.1);
        }
    }

    let mut list = if list1.is_none() { list2 } else { list1 };
    if list.is_none() {
        return reverse(result);
    }

    while list.is_some() {
        let pair = pop_front(list);
        list = pair.0;
        result = push_front(result, pair.1);
    }

    reverse(result)
}

// Add node to the front of list, then return the new list.
pub fn push_front(
    list: Option<Box<ListNode>>,
    node: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match list {
        Some(list) => match node {
            Some(mut node) => {
                node.next = Some(list);
                Some(node)
            }
            None => Some(list),
        },
        None => node,
    }
}

fn reverse(list: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut list = list;
    let mut result = None;
    while list.is_some() {
        let pair = pop_front(list);
        list = pair.0;
        result = push_front(result, pair.1);
    }

    result
}

// Returns (list without first node, popped node)
fn pop_front(list: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
    match list {
        Some(list) => {
            let value = list.val;
            let popped_node = Box::new(ListNode {
                val: value,
                next: None,
            });
            (list.next, Some(popped_node))
        }
        None => (None, None),
    }
}

fn get_some_node_value(node: &Option<Box<ListNode>>) -> i32 {
    debug_assert!(node.is_some());
    match node {
        Some(node) => node.val,
        None => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::solution;
    use crate::test_utility;

    const REVERSE_PARAMETER_1: [&[i32]; 4] = [&[1, 2, 3, 4, 5], &[], &[1], &[10, 11]];
    const REVERSE_EXPECTED_RESULT: [&[i32]; 4] = [&[5, 4, 3, 2, 1], &[], &[1], &[11, 10]];
    const POP_FRONT_PARAMETER_1: [&[i32]; 4] = [&[], &[1], &[1, 2], &[1, 2, 3]];
    // Nodes remaining in input list after the first node is popped.
    const POP_FRONT_EXPECTED_RESULT: [&[i32]; 4] = [&[], &[], &[2], &[2, 3]];
    const PUSH_FRONT_PARAMETER_1: [&[i32]; 5] = [&[2, 3, 4], &[], &[1], &[], &[2]];
    const PUSH_FRONT_PARAMETER_2: [&[i32]; 5] = [&[1], &[], &[], &[5], &[1]];
    const PUSH_FRONT_EXPECTED_RESULT: [&[i32]; 5] = [&[1, 2, 3, 4], &[], &[1], &[5], &[1, 2]];

    #[test]
    fn test_reverse() {
        assert_eq!(REVERSE_PARAMETER_1.len(), REVERSE_EXPECTED_RESULT.len());
        for index in 0..REVERSE_PARAMETER_1.len() {
            let parameter_1 = REVERSE_PARAMETER_1[index];
            let list = test_utility::create_list(parameter_1.to_vec());
            let result = solution::reverse(list);
            let result = test_utility::get_list_values(&result);
            let expected_result = REVERSE_EXPECTED_RESULT[index];
            assert_eq!(result, expected_result);
        }
    }

    #[test]
    fn test_pop_front() {
        assert_eq!(POP_FRONT_PARAMETER_1.len(), POP_FRONT_EXPECTED_RESULT.len());
        for index in 0..POP_FRONT_PARAMETER_1.len() {
            let parameter_1 = POP_FRONT_PARAMETER_1[index];
            let list = test_utility::create_list(parameter_1.to_vec());
            let pair = solution::pop_front(list);
            let result = test_utility::get_list_values(&pair.0);
            let expected_result = POP_FRONT_EXPECTED_RESULT[index];
            assert_eq!(result, expected_result);
            if !expected_result.is_empty() {
                let popped_value = test_utility::get_list_values(&pair.1)[0];
                let first_value = parameter_1[0];
                assert_eq!(popped_value, first_value);
            }
        }
    }

    #[test]
    fn test_push_front() {
        assert_eq!(PUSH_FRONT_PARAMETER_1.len(), PUSH_FRONT_PARAMETER_2.len());
        assert_eq!(
            PUSH_FRONT_PARAMETER_2.len(),
            PUSH_FRONT_EXPECTED_RESULT.len()
        );
        for index in 0..PUSH_FRONT_PARAMETER_1.len() {
            let parameter_1 = PUSH_FRONT_PARAMETER_1[index];
            let parameter_2 = PUSH_FRONT_PARAMETER_2[index];
            let list = test_utility::create_list(parameter_1.to_vec());
            let node = test_utility::create_list(parameter_2.to_vec());
            let result = solution::push_front(list, node);
            let result = test_utility::get_list_values(&result);
            let expected_result = PUSH_FRONT_EXPECTED_RESULT[index];
            assert_eq!(result, expected_result);
        }
    }
}
