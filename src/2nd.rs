#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl ListNode<T> {
    #[inline]
    fn new(val: T) -> Self {
        ListNode { next: None, val }
    }
}
