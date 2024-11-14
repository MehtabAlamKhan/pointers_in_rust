// mod ll;
// // mod ll1;
mod BinaryTree;

// use ll::ListNode;
// use std::borrow::BorrowMut;
// // use ll1::LinkedList;

use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use BinaryTree::TreeNode;

fn main() {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        let mut q: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![root];
        while !q.is_empty() {
            for _ in 0..q.len() {
                if let Some(Some(node)) = q.pop() {
                    let mut bowrrowed_node = node.borrow_mut();
                    if bowrrowed_node.left.is_some() {
                        q.push(bowrrowed_node.left.take());
                    }
                    if bowrrowed_node.right.is_some() {
                        q.push(bowrrowed_node.right.take());
                    }
                }
            }
            res += 1;
        }
        res
    }
}
