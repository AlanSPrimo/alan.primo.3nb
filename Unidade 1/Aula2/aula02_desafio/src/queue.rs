use std::ptr;
use crate::node::Node; // Importa o Node<T> do módulo node.rs

pub struct Queue<T> {
    head: Option<Box<Node<T>>>,
    tail: *mut Node<T>,
    len: usize,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { head: None, tail: ptr::null_mut(), len: 0 }
    }

    pub fn enqueue(&mut self, elem: T) {
        let mut new_node = Box::new(Node { value: elem, next: None });
        let raw_node: *mut _ = &mut *new_node;

        if self.tail.is_null() {
            self.head = Some(new_node);
        } else {
            unsafe { (*self.tail).next = Some(new_node); }
        }

        self.tail = raw_node;
        self.len += 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|head_node| {
            self.head = head_node.next;
            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }
            self.len -= 1;
            head_node.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while self.dequeue().is_some() {}
    }
}