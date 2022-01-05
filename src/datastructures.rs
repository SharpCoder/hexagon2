/*
    Author: Josh Cole
    
    This is a rust implementation of a system level linked list. It supports
    standard operations including pop, and get.
*/
#![allow(dead_code)]
use crate::mem::{ kalloc, free };


pub trait Stack <T> {
    fn push(&mut self, item: T);
    fn pop(&mut self) -> Option<T>;
}

pub trait Queue <T> {
    fn enqueue(&mut self, item: T);
    fn dequeue(&mut self) -> Option<T>;
}

/**
Vector is a heap-backed datastructure
which allocates dynamic memory and implements Stack.
*/
#[derive(Copy, Clone)]
pub struct Node<T : Copy> {
    item: T,
    next: Option<*mut Node<T>>,
}

pub struct Vector<T : Copy> {
    pub head: Option<*mut Node<T>>,
    pub size: usize,
}

impl <T: Copy> Stack<T> for Vector<T> {
    fn push(&mut self, item: T) {
        let ptr = kalloc();
        unsafe {
            (*ptr) = Node {
                item: item,
                next: self.head,
            };
        }

        self.head = Some(ptr);
        self.size = self.size + 1;
    }

    fn pop(&mut self) -> Option<T> {
        match self.head {
            None => {
                return None;
            },
            Some(node) => {
                // Copy the reference
                let indirect = node.clone();
                let node_item = unsafe { *indirect };
                
                // Free the actual node.
                free(node);

                let result = node_item.item;
                self.head = node_item.next;
                self.size = self.size - 1;
                return Some(result);
            },
        };  
    }
}
impl <T: Copy> Vector<T> {
    fn new() -> Self {
        return Vector { head: None, size: 0 };
    }

    pub fn size(&self) -> usize {
        return self.size;
    }
}

/**
Buffer is a data structure that supports
stack and queue operations, but is
a fixed length and does not use extra
memory.
*/
pub struct Buffer<const SIZE: usize, T> {
    pub data: [T; SIZE],
    pub tail: usize,
}

impl <const SIZE: usize, T : Copy> Stack<T> for Buffer<SIZE, T> {
    fn push(&mut self, item: T) {
        self.data[self.tail] = item;
        self.tail += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.tail == 0 {
            return None;
        }

        let item = self.data[self.tail - 1];
        self.tail -= 1;
        return Some(item);
    }
}

impl <const SIZE: usize, T : Copy> Queue<T> for Buffer<SIZE, T> {
    fn enqueue(&mut self, item: T) {
        self.data[self.tail] = item;
        self.tail += 1;
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.tail == 0 {
            return None;
        }

        let result = self.data[0];

        // Shift everything to the left
        for idx in 0 .. self.tail {
            self.data[idx] = self.data[idx + 1];
        }

        self.tail -= 1;

        return Some(result);
    }
}

impl <const SIZE: usize, T : Copy> Buffer<SIZE, T> {
    pub fn new(default: T) -> Self {
        return Buffer {
            data: [default; SIZE],
            tail: 0,
        }
    }

    pub fn size(&self) -> usize {
        return self.tail;
    }

    pub fn as_array(&self) -> [T; SIZE] {
        return self.data;
    }
}


#[cfg(test)]
mod test { 

    use super::*;

    #[test]
    fn stack() {
        let mut list = Vector::new();
        list.push(32);
        list.push(64);
        list.push(128);
        list.push(256);

        assert_eq!(list.size(), 4);
        assert_eq!(list.pop(), Some(256));
        assert_eq!(list.size(), 3);
        assert_eq!(list.pop(), Some(128));
        assert_eq!(list.size(), 2);
        assert_eq!(list.pop(), Some(64));
        assert_eq!(list.size(), 1);
        assert_eq!(list.pop(), Some(32));
        assert_eq!(list.size(), 0);
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn buffer() {
        let mut buffer = Buffer::<10, u8>::new(0);
        buffer.enqueue(32);
        buffer.enqueue(64);
        buffer.enqueue(128);
        assert_eq!(buffer.dequeue(), Some(32));
        assert_eq!(buffer.dequeue(), Some(64));
        assert_eq!(buffer.dequeue(), Some(128));
        assert_eq!(buffer.dequeue(), None);

        buffer.enqueue(32);
        buffer.enqueue(64);
        buffer.push(128);
        assert_eq!(buffer.pop(), Some(128));
        assert_eq!(buffer.pop(), Some(64));
        assert_eq!(buffer.pop(), Some(32));
    }
}