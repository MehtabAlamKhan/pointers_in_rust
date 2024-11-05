use std::fmt::Debug;

#[derive(Debug)]
struct Node<T: Debug> {
    value: T,
    next: Option<Box<Node<T>>>,
}
struct LinkedList<T: Debug> {
    head: Option<Box<Node<T>>>,
    tail: Option<*mut Node<T>>,
}

impl<T: Debug> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
        }
    }
    fn prepend(&mut self, value: T) {
        let mut new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        if self.tail.is_none() {
            let tail_ptr: *mut _ = &mut *new_node;
            self.tail = Some(tail_ptr)
        }
        self.head = Some(new_node);
    }
    fn pop_first(&mut self) -> Option<T> {
        match self.head.take() {
            Some(head_node) => {
                self.head = head_node.next;

                if self.head.is_none() {
                    self.tail = None;
                }
                Some(head_node.value)
            }
            None => None,
        }
    }
    fn print_tail_address(&self) {
        if let Some(tail_ptr) = self.tail {
            println!("Address is {:?} ", tail_ptr);
            unsafe {
                println!("Value is {:?} ", *tail_ptr);
            }
        } else {
            println!("None")
        }
    }

    fn print(&self) {
        let mut current = &self.head;
        while let Some(ref node) = current {
            print!("{:?} -> ", node.value);
            current = &node.next
        }
        println!("None");
    }
}

fn main() {
    let mut list1: LinkedList<i32> = LinkedList::new();
    list1.prepend(32);
    list1.prepend(4);
    list1.prepend(89);
    list1.print();
    list1.print_tail_address();
    list1.pop_first();
    list1.print();
    list1.print_tail_address();
    list1.pop_first();
    list1.pop_first();
    list1.print();
    list1.print_tail_address();
    list1.prepend(8);
    list1.print();
    list1.print_tail_address();
}
