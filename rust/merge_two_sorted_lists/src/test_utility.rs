use crate::list_node;
use crate::solution;

#[allow(dead_code)]
pub fn create_list(mut numbers: Vec<i32>) -> Option<Box<list_node::ListNode>> {
    numbers.reverse();
    let mut result = None;
    for number in numbers {
        let node = list_node::ListNode { val: number, next: None };
        result = solution::push_front(result, Some(Box::new(node)));
    }

    result
}

#[allow(dead_code)]
pub fn get_list_values(list: &Option<Box<list_node::ListNode>>) -> Vec<i32> {
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