use crate::list_node::ListNode;

pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut result = None;
    loop {
        if list1.is_none() || list2.is_none() {
            break;
        }

        let value_1: i32;
        let list1 = match list1 {
            Some(ref node) => {
                value_1 = node.val;
                Some(node);
            },
            None => panic!()
        };

        let value_2: i32;
        let list2 = match list2 {
            Some(ref node) => {
                value_2 = node.val;
                Some(node);
            },
            None => panic!()
        };

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