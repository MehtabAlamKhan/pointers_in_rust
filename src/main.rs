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
        let mut q: Vec<(Option<Rc<RefCell<TreeNode>>>, i32)> =
            vec![(root.clone(), root.clone().unwrap().borrow_mut().val)];
        let mut res = 0;
        while !q.is_empty() {
            // let (node, max_val) = q.pop();
        }
    }
}
