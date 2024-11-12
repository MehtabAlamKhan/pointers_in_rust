// mod ll;
// mod ll1;
mod BinaryTree;

use std::borrow::BorrowMut;
// use ll::ListNode;
// use ll1::LinkedList;
use std::cell::RefCell;
use std::rc::Rc;
use BinaryTree::TreeNode;

fn main() {
    let mut tree = TreeNode::new(4);
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root == None {
            return root;
        }

        std::mem::swap(
            root.unwrap().borrow().left.borrow_mut(),
            root.unwrap().borrow().right.borrow_mut(),
        );

        return root;
    }
}
