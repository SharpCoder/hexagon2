use crate::system::vector::*;

pub type String = Vector::<u8>;

pub trait StringBuffer {
    fn index_of(&self, target: String) -> Option<usize>;
    fn contains(&self, target: String) -> bool;
    fn split(&self, separator: u8) -> Vector::<String>;
}

impl StringBuffer for Vector::<u8> {
    fn index_of(&self, target: String) -> Option<usize> {
        if target.size() == 0 {
            return None;
        } else if self.size() == 0 {
            return None;
        } else if self.size() < target.size() {
            return None;
        }
    
        for i in 0 .. self.size() - target.size() {
            if self.get(i) == target.get(0) {
                let mut found = true;
                for r in 0 .. target.size() {
                    if self.get(i + r) != target.get(r) {
                        found = false;
                        break;
                    }
                }
    
                if found {
                    return Some(i);
                }
            }
        }
        return None;
    }

    fn contains(&self, target: String) -> bool {
        return self.index_of(target).is_some();
    }

    fn split(&self, separator: u8) -> Vector::<String> {
        let mut original = self.clone();
        let mut result = Vector::new();
        let mut temp = Vector::new();

        for _ in 0 .. self.size() {
            match original.dequeue() {
                None => {},
                Some(byte) => {
                    if byte == separator {
                        result.push(temp.clone());
                        temp.clear();
                    } else {
                        temp.push(byte);
                    }
                }
            }            
        }

        if temp.size() > 0 {
            result.push(temp.clone());
        }


        return result;
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    fn vecs_eq(left: Vector::<u8>, right: Vector::<u8>) {
        assert_eq!(left.size(), right.size());
        for idx in 0 .. left.size() {
            assert_eq!(left.get(idx), right.get(idx));
        }
    }

    #[test]
    fn test_str_contains() {
        assert_eq!(vec_str!(b"WIFI DISCONNECT").contains(vec_str!(b"WIFI GOT IP")), false);
        assert_eq!(vec_str!(b"hello world").contains(vec_str!(b"wo")), true);
        assert_eq!(vec_str!(b"hello world").contains(vec_str!(b"woldz")), false);
        assert_eq!(vec_str!(b"hello world").contains(vec_str!(b"")), false);
        assert_eq!(vec_str!(b"").contains(vec_str!(b"woldz")), false);
        assert_eq!(vec_str!(b" ").contains(vec_str!(b"woldz")), false);
        assert_eq!(vec_str!(b"     ").contains(vec_str!(b"woldz")), false);
    }

    #[test]
    fn test_index_of() {
        assert_eq!(vec_str!(b"+CIPSTATUS:23").index_of(vec_str!(b":")), Some(10));
        assert_eq!(vec_str!(b"+CIPSTATUS:23").index_of(vec_str!(b"+")), Some(0));
        assert_eq!(vec_str!(b"+CIPSTATUS:23").index_of(vec_str!(b"234")), None);
    }

    #[test]
    fn test_split() {
        let text = vec_str!(b"Hello\nHow\nAre\nYou?");
        let words= text.split(b'\n');
        assert_eq!(words.size(), 4);
        vecs_eq(words.get(0).unwrap(), vec_str!(b"Hello"));
    }
}