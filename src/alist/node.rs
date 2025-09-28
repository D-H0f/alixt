#![allow(unused)]
pub(super) use std::cell::RefCell;
pub(super) use std::rc::{Weak, Rc};


pub(super) type StrongNode<T> = Rc<RefCell<Node<T>>>;
pub(super) type WeakNode<T> = Weak<RefCell<Node<T>>>;

pub(super) struct Node<T> {
    data: Option<T>,
    prev: Option<WeakNode<T>>,
    next: Option<StrongNode<T>>,
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        let mut next = self.next.take();
        while let Some(rc) = next {
            next = match Rc::try_unwrap(rc) {
                Ok(node) => {
                    node.into_inner().take_next()
                },
                Err(_) => {
                    eprintln!(
                        "Some node was stopped from dropping by another owner."
                    );
                    break;
                },
            }
        }
    }
}

impl<T> Node<T> {
    pub(super) fn from(
        data: Option<T>,
        prev: Option<StrongNode<T>>,
        next: Option<StrongNode<T>>,
    ) -> StrongNode<T> {
        Rc::new(RefCell::new(Node {
            data,
            prev: prev.as_ref().map(Rc::downgrade),
            next,
        }))
    }
    /// constructs a new Node with no links
    pub(super) fn new(data: T) -> StrongNode<T> {
        Node::from(Some(data), None, None)
    }
    /// constructs a new Node to be used as a head
    pub(super) fn new_head(data: T, next: Option<StrongNode<T>>) -> StrongNode<T> {
        Node::from(Some(data), None, next)
    }
    /// constructs a new Node to be used as a tail
    pub(super) fn new_tail(data: T, prev: Option<StrongNode<T>>) -> StrongNode<T> {
        Node::from(Some(data), prev, None)
    }
    // prev node helpers
    /// returns a new Rc pointing to the node in prev of this one
    pub(super) fn prev(&self) -> Option<StrongNode<T>> {
        self.prev.as_ref().and_then(|weak| weak.upgrade())
    }
    pub(super) fn take_prev(&mut self) -> Option<StrongNode<T>> {
        self.prev.take().and_then(|weak| weak.upgrade())
    }
    pub(super) fn swap_prev(&mut self, other: &mut Node<T>) {
        std::mem::swap(&mut self.prev, &mut other.prev);
    } 
    pub(super) fn set_prev(&mut self, prev: Option<StrongNode<T>>) {
        self.prev = prev.as_ref().map(Rc::downgrade)
    }
    pub(super) fn clear_prev(&mut self) {
        self.prev.take();
    }
    // next node helpers
    /// returns a new Rc pointing to the node behind this one
    pub(super) fn next(&self) -> Option<StrongNode<T>> {
        self.next.clone()
    }
    pub(super) fn take_next(&mut self) -> Option<StrongNode<T>> {
        self.next.take()
    }
    pub(super) fn swap_next(&mut self, other: &mut Node<T>) {
        std::mem::swap(&mut self.next, &mut other.next);
    }
    pub(super) fn set_next(&mut self, next: Option<StrongNode<T>>) {
        self.next = next;
    }
    pub(super) fn clear_next(&mut self) {
        self.next.take();
    }
    // data helpers
    pub(super) fn data(&self) -> &T {
        self.data.as_ref().unwrap()
    }
    pub(super) fn into_data(mut self) -> T {
        self.data.take().unwrap()
    }
    pub(super) fn replace_data(&mut self, data: T) -> T {
        std::mem::replace(self.data.as_mut().unwrap(), data)
    }
    pub(super) fn switch(&mut self) {
        let next = self.prev.take().and_then(|weak| weak.upgrade());
        let prev = self.next.take().map(|rc| Rc::downgrade(&rc));
        self.prev = prev;
        self.next = next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_creation() {
        let n1 = Node::new(24u8);

        assert_eq!(*n1.borrow().data(), 24);
        assert!(n1.borrow().next().is_none());
        assert!(n1.borrow().next().is_none());
    }
}
