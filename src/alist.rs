#![allow(unused)]
mod node;
use node::*;

#[derive(Default)]
pub struct AList<T> {
    head: Option<StrongNode<T>>,
    tail: Option<WeakNode<T>>,
    len: usize,
}

impl<T> AList<T> {
    pub fn new() -> Self {
        AList { head: None, tail: None, len: 0 }
    }
    pub fn from(data: T) -> Self {
        let node = Node::new(data);
        AList {
            tail: Some(Rc::downgrade(&node)),
            head: Some(node),
            len: 1,
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn map_head<C: FnOnce(&T) -> R, R>(&self, closure: C) -> Option<R> {
        self.head.as_ref().map(|rc| {
            closure(rc.borrow().data())
        })
    }
    pub fn push(&mut self, data: T) {
        let mut new_head = Node::new_head(data, self.head.clone());
        if let Some(node) = self.head.as_mut() {
            node.borrow_mut().set_prev(Some(Rc::clone(&new_head)));
        } else {
            self.tail = Some(Rc::downgrade(&new_head));
        }
        self.head = Some(new_head);
        self.len += 1;
    }
    pub fn pop(&mut self) -> Option<T> {
        let mut node = Rc::try_unwrap(self.head.take()?)
            .map(|wrapper| { wrapper.into_inner() })
            .unwrap_or_else(|_| {
                panic!("head should have no other owners. If you see this, it's a bug.")
            });
        self.head = node
            .take_next()
            .inspect(|new_head| {
                new_head.borrow_mut().clear_prev();
            });
        if self.head.is_none() {
            self.tail = None
        }
        self.len -= 1;
        Some(node.into_data())
    }
    pub fn push_back(&mut self, data: T) {
        match self.tail.as_mut() {
            Some(tail) => {
                let old_tail = tail.upgrade();
                let node = Node::new_tail(data, old_tail.clone());
                self.tail = Some(Rc::downgrade(&node));
                old_tail.unwrap().borrow_mut().set_next(Some(node.clone()));
            },
            None if self.head.is_none() => {
                let node = Node::new(data);
                self.tail = Some(Rc::downgrade(&node));
                self.head = Some(node);
            },
            None => { panic!("If tail is None, head should also be None. If you reached this, it's a bug.")},
        }
        self.len += 1;
    }
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.as_ref()?;
        let mut tail = self
            .tail
            .take()
            .and_then(|weak| weak.upgrade())
            .unwrap();
        let new_tail = tail.borrow_mut().take_prev();
        if new_tail.is_none() { self.head.take(); }
        self.tail = new_tail.as_ref().map(|rc| {
            rc.borrow_mut().clear_next();
            Rc::downgrade(rc)
        });
        self.len -= 1;
        Some(Rc::try_unwrap(tail)
            .unwrap_or_else(|_| {
                panic!("Nothing should own this node, if you see this it's a bug")
            })
            .into_inner()
            .into_data())
    }
    pub fn reversed(&mut self) {
        let mut current = self
            .head
            .take()
            .inspect(|rc| {
                self.tail = Some(Rc::downgrade(&rc.clone()));
            });
        let mut prev = None;
        while let Some(current_rc) = current {
            let mut node = current_rc.borrow_mut();
            current = node.next().clone();
            node.switch();
            prev = Some(current_rc.clone());
        };
        self.head = prev;
    }
    /*pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.clone() }
    }*/
}

pub struct IntoIter<T>(AList<T>);

impl<T> IntoIterator for AList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

// not planned until AList is more fleshed out and ready to be converted to unsafe
/*
pub struct Iter<T> {
    next: Option<StrongNode<T>>,
}

impl<T> Iterator for Iter<T> {
    type Item = StrongNode<T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|rc| {
            self.next = rc.borrow().next();
            rc.clone()
        })
    }
}*/





use std::{any::type_name_of_val, fmt};
impl<T: fmt::Debug> fmt::Debug for AList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str("placeholder");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::AList;
    #[test]
    fn placeholder() {
        let list: AList<u8> = AList::new();
        println!("{:?} {:#?}", list, list);
    }
}
