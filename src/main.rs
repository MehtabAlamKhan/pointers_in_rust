mod ll;
// mod ll1;
mod BinaryTree;

use ll::ListNode;
use std::borrow::BorrowMut;
// use ll1::LinkedList;
use std::cell::RefCell;
use std::rc::Rc;
use BinaryTree::TreeNode;

fn main() {
    let mut tree = TreeNode::new(4);

    let mut l1 = &mut Some(Box::new(ListNode::new(1)));
    l1.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    println!("{:?}", l1);

    // pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    //     if root == None {
    //         return root;
    //     }

    //     // std::mem::swap(
    //     //     root.unwrap().borrow().left.borrow_mut(),
    //     //     root.unwrap().borrow().right.borrow_mut(),
    //     // );

    //     return root;
    // }
}
