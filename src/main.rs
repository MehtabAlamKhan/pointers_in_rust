// mod ll;
// // mod ll1;
mod BinaryTree;
mod Heap;

// use ll::ListNode;
// // use ll1::LinkedList;

use std::cell::RefCell;
use std::rc::Rc;
use BinaryTree::TreeNode;

fn main() {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        (&mut res, root);
        res
    }
    pub fn helper(res: &mut i32, root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node_rc) = root {
            let left = (0).max(helper(res, node_rc.borrow().left.clone()));
            let right = (0).max(helper(res, node_rc.borrow().right.clone()));
            *res = (*res).max(node_rc.borrow().val + left + right);
            return node_rc.borrow().val + left.max(right);
        } else {
            0
        }
    }
}
