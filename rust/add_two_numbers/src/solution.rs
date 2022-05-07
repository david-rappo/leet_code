#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None
        }
    }
}

// This page shows one way to create a singly linked list in safe Rust:
// https://stackoverflow.com/questions/54015194/two-sum-leetcode-rust-solution
//
// Learn Rust With Entirely Too Many Linked Lists
// https://rust-unofficial.github.io/too-many-lists/
//
// std::mem::replace could also be useful when implementing Linked Lists in Rust.
// https://doc.rust-lang.org/std/mem/fn.replace.html
#[allow(dead_code)]
pub fn create_list(digits: &[i32]) -> Option<Box<ListNode>> {
    if digits.is_empty() {
        return None;
    }

    let mut head_node = Some(Box::new(ListNode::new(digits[0])));
    let mut node = &mut head_node;
    for index in 1..digits.len() {
        node = match node.as_mut() {
            Some(match_node) => {
                let new_node = Box::new(ListNode::new(digits[index]));
                match_node.next = Some(new_node);
                &mut match_node.next
            },
            _ => unreachable!()
        };
    }
    
    head_node
}

// l1 = 243
// l2 = 564
// result = 708
//
// 342 + 465 = 807
//
// 243
// 564
// ---
// 708
#[allow(dead_code)]
pub fn add_two_numbers(l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut list_1 = l1.as_ref();
    let mut list_2 = l2.as_ref();
    let mut vec_digits = vec![];
    let mut carry: Option<i32> = None;
    loop {
        let x = {
            if list_1.is_some() {
                list_1.unwrap().val
            } else {
                0
            }
        };

        let y = {
            if list_2.is_some() {
                list_2.unwrap().val
            } else {
                0
            }
        };

        if list_1.is_none() && list_2.is_none() {
            break;
        }

        // Less than 10, sum = sum, carry = None
        //           10: sum = 0, carry 1
        //           11: sum = 1, carry 1
        //           19: sum = 9, carry 1
        let mut sum = x + y;
        if carry.is_some() {
            sum += carry.unwrap();
        }

        if sum > 9 {
            carry = Some(1);
            sum = sum - 10;
        } else {
            carry = None;
        }

        vec_digits.push(sum);

        if list_1.is_some() {
            list_1 = list_1.unwrap().next.as_ref();
        }

        if list_2.is_some() {
            list_2 = list_2.unwrap().next.as_ref();
        }
    }

    // 9 +
    // 9
    // --
    // 81

    if carry.is_some() {
        vec_digits.push(1);
    }
    
    create_list(&vec_digits)
}