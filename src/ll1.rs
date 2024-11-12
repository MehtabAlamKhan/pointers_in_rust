use std::fmt::Debug;

#[derive(Debug)]
pub struct Node<T: Debug> {
    value: T,
    next: Option<Box<Node<T>>>,
}
pub struct LinkedList<T: Debug> {
    head: Option<Box<Node<T>>>,
    tail: Option<*mut Node<T>>,
}

impl<T: Debug> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
        }
    }
    pub fn insert_at_pos(&mut self, value: T, pos: usize) {
        if pos == 0 {
            self.prepend(value);
            println!("ADDED AT {} POSITION", pos);
            self.print();
            return;
        }
        let mut new_node = Box::new(Node { value, next: None });
        let mut cur = &mut self.head;
        let mut i = 0;
        while let Some(ref mut node) = cur {
            if i == pos - 1 {
                new_node.next = node.next.take();
                node.next = Some(new_node);
                if node.next.as_ref().unwrap().next.is_none() {
                    self.tail = Some(&mut **node.next.as_mut().unwrap());
                }
                return;
            }
            cur = &mut node.next;
            i += 1;
        }
        println!("INVALID POSITION")
    }
    pub fn prepend(&mut self, value: T) {
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
    pub fn append(&mut self, value: T) {
        if self.tail.is_none() {
            self.prepend(value);
            return;
        }
        let new_node = Box::new(Node { value, next: None });
        unsafe {
            (*self.tail.unwrap()).next = Some(new_node);
            self.tail = Some((*self.tail.unwrap()).next.as_mut().unwrap().as_mut());
            return;
        }
    }
    pub fn pop_last(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        if self.head.as_ref()?.next.is_none() {
            return self.pop_first();
        }
        let mut cur = &mut self.head;
        while let Some(ref mut node) = cur {
            if node.next.as_ref()?.next.is_none() {
                let last_node = node.next.take();
                self.tail = Some(&mut **node);
                return Some(last_node?.value);
            }
            cur = &mut node.next;
        }
        None
    }
    pub fn pop_first(&mut self) -> Option<T> {
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
    pub fn print_tail_address(&self) {
        if let Some(tail_ptr) = self.tail {
            println!("Address is {:?} ", tail_ptr);
            unsafe {
                println!("Value is {:?} ", *tail_ptr);
            }
        } else {
            println!("None")
        }
    }
    pub fn print(&self) {
        let mut current = &self.head;
        println!("CURRENT NODES IN THE LIST ARE -> ");
        while let Some(ref node) = current {
            print!("{:?} -> ", node.value);
            current = &node.next
        }
        println!("None");
    }
}
