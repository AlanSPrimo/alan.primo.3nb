use std::option::Option;

pub struct Node <T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}