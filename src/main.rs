// mod ll;
// // mod ll1;
mod BinaryTree;

// use ll::ListNode;
// use std::borrow::BorrowMut;
// // use ll1::LinkedList;

use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::rc::Rc;
use BinaryTree::TreeNode;

fn main() {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut q: Vec<Vec<(Option<Rc<RefCell<TreeNode>>>, i32)>> = vec![vec![root, root.unwrap().borrow_mut().]];
        let mut res = 0;
        while !q.is_empty() {}
        return 3;
    }
}
