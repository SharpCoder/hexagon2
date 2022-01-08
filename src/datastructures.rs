#![allow(dead_code)]
use crate::mem::{ kalloc, free };

/// This macro returns a vector of the items you pass to it.
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vector::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

/// This macro takes a static string and returns
/// a vector containing the sequence of characters.
#[macro_export]
macro_rules! vec_str {
    ($arr:tt) => {
        Vector::from_slice($arr)
    };
}

/// A string, also known as a collection of bytes.
pub type String = Vector::<u8>;

pub trait Stack <T> {
    fn push(&mut self, item: T);
    fn pop(&mut self) -> Option<T>;
}

pub trait Queue <T> {
    fn enqueue(&mut self, item: T);
    fn dequeue(&mut self) -> Option<T>;
}

pub trait Array<T> {
    fn get(&self, index: usize) -> Option<T>;
    fn get_mut(&mut self, index: usize) -> Option<&mut T>;
    fn size(&self) -> usize;
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

impl <T: Clone + Copy> Clone for Vector<T> {
    fn clone(&self) -> Self {
        if self.head.is_none() {
            return Vector::new();
        }
        
        let mut result = Vector::new();
        let mut ptr = self.head.unwrap();

        loop {
            let item = unsafe { (*ptr).item };
            result.enqueue(item);

            if unsafe { *ptr }.next.is_some() {
                ptr = unsafe { *ptr }.next.unwrap();
            } else {
                break;
            }
        }

        return result;
    }
}

impl <T: Clone + Copy> Copy for Vector<T> {
    
}

impl <T: Clone + Copy> Array<T> for Vector<T> {
    fn size(&self) -> usize {
        return self.size;
    }

    fn get(&self, index: usize) -> Option<T> {
        if self.head.is_none() || index >= self.size {
            return None;
        } else {
            // Travel n times through the linked list
            let mut ptr = self.head.unwrap();
            for _ in 0 .. index {
                ptr = unsafe { *ptr }.next.unwrap();
            }
            return unsafe { Some((*ptr).item) };
        }
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if self.head.is_none() || index >= self.size {
            return None;
        } else {
            // Travel n times through the linked list
            let mut ptr = self.head.unwrap();
            for _ in 0 .. index {
                ptr = unsafe { *ptr }.next.unwrap();
            }
            return unsafe { Some(&mut (*ptr).item) };
        }
    }
}

impl <T: Clone + Copy> Queue<T> for Vector<T> {
    fn enqueue(&mut self, item: T) {
        // Add it to the end of the stack
        let ptr = kalloc();
        unsafe {
            (*ptr) = Node {
                item: item,
                next: None,
            }
        }

        if self.head.is_none() {
            self.head = Some(ptr);
        } else {
            let mut tail_ptr = self.head.unwrap();
    
            // Find the tail
            while unsafe { *tail_ptr }.next.is_some() {
                tail_ptr = unsafe { (*tail_ptr).next.unwrap() };
            }
    
            unsafe { (*tail_ptr).next = Some(ptr) };
        }
        self.size += 1;

    }

    fn dequeue(&mut self) -> Option<T> {
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

impl <T: Clone + Copy> Stack<T> for Vector<T> {
    fn push(&mut self, item: T) {
        self.enqueue(item);
    }

    fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        let node_item;

        if self.size == 1 {
            // Return head node
            node_item = unsafe { *self.head.unwrap() }.item;
            self.head = None;

        } else {
            // Travel to the correct node
            let mut ptr = self.head.unwrap();
            for _ in 0 .. (self.size() - 2) {
                ptr = unsafe { (*ptr).next.unwrap() };
            }
            
            node_item = unsafe { (*(*ptr).next.unwrap()).item };
            unsafe { (*ptr).next = None };
        }

        self.size -= 1;
        return Some(node_item);
    }
}
impl <T: Clone + Copy> Vector<T> {
    pub fn new() -> Self {
        return Vector { head: None, size: 0 };
    }

    pub fn from_slice(items: &[T]) -> Self {
        let mut result = Vector::new();
        for item in items {
            result.enqueue(item.clone());
        }
        return result;
    }

    pub fn size(&self) -> usize {
        return self.size;
    }

    pub fn join(&mut self, vec_to_join: Vector<T>) -> &mut Self {
        for index in 0 .. vec_to_join.size() {
            self.enqueue(vec_to_join.get(index).unwrap());
        }
        return self;
    }

    pub fn substr(&self, start: usize, length: usize) -> Option<Self> {
        let mut result = Vector::new();
        if start + length > self.size() {
            return None;
        }

        for idx in start .. (start + length) {
            result.enqueue(self.get(idx).unwrap());
        }

        return Some(result);
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
        if self.tail == SIZE {
            // Discard the data. we are buffer oerflow.
            return;
        }
        
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
        if self.tail == SIZE {
            // Discard the data. we are buffer oerflow.
            return;
        }

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
            self.data[idx] = self.data[idx + 1].clone();
        }

        self.tail -= 1;

        return Some(result);
    }
}

impl Array<u8> for &[u8] {
    fn size(&self) -> usize {
        return self.len();
    }

    fn get(&self, index: usize) -> Option<u8> {
        if index >= self.len() {
            return None;
        }
        return Some(self[index]);
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut u8> {
        if index >= self.len() {
            return None;
        }

        panic!("Not implemented");
    }
}

impl <const SIZE: usize, T : Copy> Array<T> for Buffer<SIZE, T> {
    fn size(&self) -> usize {
        return self.tail;
    }

    fn get(&self, index: usize) -> Option<T> {
        if index >= self.tail {
            return None;
        } else {
            return Some(self.data[index]);
        }
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.tail {
            return None;
        } else {
            return Some(&mut self.data[index]);
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
        return &self.data[..];
    }

    pub fn clear(&mut self) {
        self.tail = 0;
    }
}


#[cfg(test)]
mod test { 

    use super::*;

    #[derive(Copy, Clone)]
    pub struct ShadowVec {
        pub items: Vector::<u8>,
        pub time: usize,
    }

    #[test]
    fn advanced_copy() {
        let shadow = ShadowVec {
            items: Vector::from_slice(&[1,2,3,4,5]),
            time: 1337,
        };

        let next = shadow.clone();
        assert_eq!(next.items.size(), 5);
    }

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
        let mut list = Vector::<u32>::new();
        list.enqueue(32);
        list.enqueue(64);
        list.enqueue(128);
        list.enqueue(256);
        list.enqueue(512);

        assert_eq!(list.get(0), Some(32));
        assert_eq!(list.get(1), Some(64));
        assert_eq!(list.get(3), Some(256));
        assert_eq!(list.get(2), Some(128));
        assert_eq!(list.get(4), Some(512));
        assert_eq!(list.get(5), None);
        assert_eq!(list.get(100), None);

        let list2 = Vector::<i32>::new();
        assert_eq!(list2.get(0), None);
        assert_eq!(list2.get(100), None);
    }

    #[test]
    fn test_stack_clone() {
        let list = Vector::from_slice(&[32, 64, 128, 256, 512]);
        let mut cloned_list = list.clone();
        assert_eq!(cloned_list.pop(), Some(512));
        assert_eq!(cloned_list.pop(), Some(256));
        assert_eq!(cloned_list.pop(), Some(128));
        assert_eq!(cloned_list.pop(), Some(64));
        assert_eq!(cloned_list.pop(), Some(32));
        assert_eq!(cloned_list.pop(), None);

        cloned_list.join(Vector::from_slice(&[32,64]));
        let mut list3 = cloned_list.clone();
        list3.join(Vector::from_slice(&[128]));
        assert_eq!(list3.get(0), Some(32));
    }

    #[test]
    fn test_vector_queue() {
        let mut list = Vector::new();
        list.enqueue(32);
        list.enqueue(64);
        list.enqueue(128);
        
        assert_eq!(list.dequeue(), Some(32));
        assert_eq!(list.dequeue(), Some(64));
        assert_eq!(list.dequeue(), Some(128));
        assert_eq!(list.dequeue(), None);
    }

    #[test]
    fn test_vector_join() {
        let mut list1 = Vector::from_slice(&[32,64,128]);
        let list2 = Vector::from_slice(&[256,512]);
        
        list1.join(list2);

        assert_eq!(list1.pop(), Some(512));
        assert_eq!(list1.pop(), Some(256));
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