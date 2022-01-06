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

pub trait Array<T> {
    fn get(&mut self, index: usize) -> Option<T>;
}

/**
Vector is a heap-backed datastructure
which allocates dynamic memory and implements Stack.
*/
#[derive(Copy, Clone)]
pub struct Node<T : Clone + Copy> {
    item: T,
    next: Option<*mut Node<T>>,
}

pub struct Vector<T : Clone + Copy> {
    pub head: Option<*mut Node<T>>,
    pub size: usize,
}

impl <T: Clone + Copy> Array<T> for Vector<T> {
    fn get(&mut self, index: usize) -> Option<T> {
        if self.head.is_none() || index >= self.size {
            return None;
        } else {
            // Travel n times through the linked list
            let mut ptr = self.head.unwrap();
            for _ in 0 .. (self.size - index - 1) {
                ptr = unsafe { *ptr }.next.unwrap();
            }
            return unsafe { Some((*ptr).item) };
        }
    }
}

impl <T: Clone + Copy> Stack<T> for Vector<T> {
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
impl <T: Clone + Copy> Vector<T> {
    pub fn new() -> Self {
        return Vector { head: None, size: 0 };
    }

    pub fn from_slice(items: &[T]) -> Self {
        let mut result = Vector::new();
        for item in items {
            result.push(item.clone());
        }
        return result;
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

impl <const SIZE: usize, T : Copy> Array<T> for Buffer<SIZE, T> {
    fn get(&mut self, index: usize) -> Option<T> {
        if index >= self.tail {
            return None;
        } else {
            return Some(self.data[index]);
        }
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

    pub fn as_array(&self) -> &[T] {
        return &self.data[0..SIZE];
    }

    pub fn flush(&mut self) {
        self.tail = 0;
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
    fn stack_get() {
        let mut list = Vector::new();
        list.push(32);
        list.push(64);
        list.push(128);
        list.push(256);
        list.push(512);

        assert_eq!(list.get(0), Some(32));
        assert_eq!(list.get(1), Some(64));
        assert_eq!(list.get(3), Some(256));
        assert_eq!(list.get(2), Some(128));
        assert_eq!(list.get(4), Some(512));
        assert_eq!(list.get(5), None);
        assert_eq!(list.get(100), None);

        let mut list2 = Vector::<i32>::new();
        assert_eq!(list2.get(0), None);
        assert_eq!(list2.get(100), None);
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