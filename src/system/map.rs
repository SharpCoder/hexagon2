use core::cmp::*;
use crate::*;
use crate::mem::*;
use crate::system::vector::*;

pub trait Map<K : PartialOrd + PartialEq + Copy, V : Copy> {
    fn insert(&mut self, key: K, value: V);
    fn remove(&mut self, key: K);
    fn get(&self, key: K) -> Option<V>;
    fn keys(&self) -> Vector::<K>;
}

pub trait BTree<K : PartialOrd + PartialEq + Copy, V : Copy> {
    fn get_mut(&mut self, target: K) -> Option<&mut V>;
    fn get(&self, target: K) -> Option<V>;
    fn insert(&mut self, target: K, value: V);
    fn keys(&self) -> Vector::<K>;
}

// #[derive(Copy, Clone)]
struct MapNode<K : PartialOrd + PartialEq + Copy, V : Copy> {
    item: V,
    key: K,
    left: Option<*mut MapNode<K, V>>,
    right: Option<*mut MapNode<K, V>>,
}


// #[derive(Copy, Clone)]
pub struct BTreeMap<K : PartialOrd + PartialEq + Copy, V : Copy> {
    root: Option<MapNode<K, V>>,
}

impl <K : PartialOrd + PartialEq + Copy, V : Copy> MapNode<K, V> {
    pub fn new(key: K, val: V) -> *mut Self {
        let ptr = kalloc();
        unsafe {
            (*ptr) = MapNode {
                item: val,
                key: key,
                left: None,
                right: None,
            };
        }

        return ptr;
    }

    pub fn size(&self) -> usize {
        let mut result = 1;
        match self.left {
            None => {},
            Some(node) => {
                result += unsafe { node.as_ref().unwrap() }.size();
            }
        };

        match self.right {
            None => {},
            Some(node) => {
                result += unsafe { node.as_ref().unwrap() }.size();
            }
        }

        return result;
    }
}

impl <K : PartialOrd + PartialEq + Copy, V : Copy> BTree<K, V> for MapNode<K, V> {
    fn get(&self, target: K) -> Option<V> {
        if self.key == target {
            return Some(self.item);
        } else if self.key > target {
            // Go left
            return match self.left {
                None => None,
                Some(node) => unsafe { node.as_ref().unwrap() }.get(target) 
            };
        } else {
            // Go right
            return match self.right {
                None => None,
                Some(node) => unsafe { node.as_ref().unwrap() }.get(target)
            };
        }
    }

    fn get_mut(&mut self, target: K) -> Option<&mut V> {
        if self.key == target {
            return Some(&mut self.item);
        } else if self.key > target {
            // Go left
            return match self.left {
                None => None,
                Some(node) => unsafe { node.as_mut().unwrap() }.get_mut(target) 
            };
        } else {
            // Go right
            return match self.right {
                None => None,
                Some(node) => unsafe { node.as_mut().unwrap() }.get_mut(target)
            };
        }
    }

    fn insert(&mut self, target: K, value: V) {
        if target == self.key {
            return;
        } else if target < self.key {
            // Insert left
            match self.left {
                None => {
                    self.left = Some(MapNode::new(target, value));
                },
                Some(node) => {
                    unsafe { node.as_mut().unwrap() }.insert(target, value);
                }
            }
        } else {
            // Insert right
            match self.right {
                None => {
                    self.right = Some(MapNode::new(target, value));
                },
                Some(node) => {
                    unsafe { node.as_mut().unwrap() }.insert(target, value);
                }
            }
        }
    }

    fn keys(&self) -> Vector::<K> {
        let mut result = vector!(self.key);
        match self.left {
            None => {},
            Some(node) => {
                result.join(unsafe { node.as_ref().unwrap() }.keys());
            }
        }
        match self.right {
            None => {},
            Some(node) => {
                result.join(unsafe { node.as_ref().unwrap() }.keys());
            }
        }
        return result;
    }
}

impl <K : PartialOrd + PartialEq + Copy, V : Copy> BTreeMap<K, V> {
    pub fn new() -> Self {
        return BTreeMap {
            root: None,
        };
    }

    pub fn size(&self) -> usize {
        return match &self.root {
            None => 0,
            Some(node) => node.size()
        };
    }
}

impl <K : PartialOrd + PartialEq + Copy, V : Copy> Map<K, V> for BTreeMap<K, V> {
    fn insert(&mut self, key: K, value: V) {
        // If the root node is null, we can insert there
        if self.root.is_none() {
            self.root = Some(MapNode {
                key: key,
                item: value,
                left: None,
                right: None,
            });
        } else {
            self.root.as_mut().unwrap().insert(key, value);
        }
    }

    fn remove(&mut self, key: K) {
        
    }

    fn get(&self, key: K) -> Option<V> {
        return match &self.root {
            None => None,
            Some(node) => node.get(key),
        };
    }

    fn keys(&self) -> Vector::<K> {
        return match &self.root {
            None => Vector::new(),
            Some(head) => head.keys(),
        };
    }
}



#[cfg(test)]
mod test { 
    use super::*;
    use crate::system::strings::*;

    #[test]
    fn test_map_node() {
        let node = unsafe { MapNode::new(100, 50).as_mut().unwrap() };
        assert_eq!(node.size(), 1);

        node.insert(125, 25);
        assert_eq!(node.size(), 2);

        node.insert(80, 15);
        assert_eq!(node.size(), 3);

        assert_eq!(node.get(80).unwrap(), 15);
        assert_eq!(node.get(125).unwrap(), 25);
        assert_eq!(node.get(100).unwrap(), 50);
        assert_eq!(node.get(374), None);

    }

    #[test]
    fn test_btree_map() {

        let mut map = BTreeMap::<u8, u8>::new();
        map.insert(10, 1);
        map.insert(15, 2);
        map.insert(17, 3);
        
        assert_eq!(map.size(), 3);
        assert_eq!(map.get(10), Some(1));
        assert_eq!(map.get(15), Some(2));
    }

    #[test]
    fn test_btree_keys() {
        let mut map = BTreeMap::new();
        map.insert(10u8, 1u8);
        map.insert(20u8, 2u8);
        map.insert(30u8, 3u8);

        let keys = map.keys();
        assert_eq!(keys.size(), 3);
        assert_eq!(keys.get(0).unwrap(), 10u8);
        assert_eq!(keys.get(1).unwrap(), 20u8);
        assert_eq!(keys.get(2).unwrap(), 30u8);
    }
}