use std::fmt::Debug;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    pub fn new(value: i32) -> Self {
        ListNode {
            val: value,
            next: None,
        }
    }
}
