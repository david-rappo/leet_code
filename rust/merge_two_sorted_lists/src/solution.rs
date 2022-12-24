use crate::list_node::ListNode;

pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut result = None;
    let mut list1 = list1;
    let mut list2 = list2;
    loop {
        if list1.is_none() || list2.is_none() {
            break;
        }

        let value_1 = get_some_node_value(&list1);
        let value_2 = get_some_node_value(&list2);
        /*
        if list_1_pair.1 < list_2_pair.1 {
        } else {

        }
        */
        
        // TODO:
        /*
        if value_1 < value_2 {
            !todo
        } else {
            result = match result {
                Some(node) => {
                    node.
                }
                
            }
        }
        */
    }
    result
}

// Returns (list without first node, popped node)
fn pop_front(list: Option<Box<ListNode>>) ->
    (Option<Box<ListNode>>, Option<Box<ListNode>>) {
    match list {
        Some(list) => {
            let value = list.val;
            let popped_node = Box::new(ListNode { val: value, next: None });
            (list.next, Some(popped_node))
        },
        None => (None, None)
    }
}

// Add node to the front of list, then return the new list.
fn push_front(list: Option<Box<ListNode>>, node: Option<Box<ListNode>>) ->
    Option<Box<ListNode>> {
    match list {
        Some(list) => {
            match node {
                Some(mut node) => {
                    node.next = Some(list);
                    Some(node)
                },
                None => Some(list)
            }
        },
        None => node
    }
}

fn get_some_node_value(node: &Option<Box<ListNode>>) -> i32 {
    debug_assert!(node.is_some());
    match node {
        Some(node) => {
            node.val
        },
        None => panic!()
    }
}