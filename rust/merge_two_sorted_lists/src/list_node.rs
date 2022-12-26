// ListNode cannot be modified. It is defined by Leet Code.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}
