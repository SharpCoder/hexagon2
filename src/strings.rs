use crate::datastructures::*;

pub fn concat(first: &'static [u8], second: &'static [u8]) -> Vector<u8> {
    let mut temp = Vector::new();
    for idx in 0 .. first.len() {
        temp.enqueue(first[idx]);
    }
    for idx in 0 .. second.len() {
        temp.enqueue(second[idx]);
    }
    return temp;
}

pub fn index_of(buffer: &dyn Array<u8>, target: &dyn Array<u8>) -> Option<usize> {
    if target.size() == 0 {
        return None;
    } else if buffer.size() == 0 {
        return None;
    } else if buffer.size() < target.size() {
        return None;
    }

    for i in 0 .. buffer.size() - target.size() {
        if buffer.get(i) == target.get(0) {
            let mut found = true;
            for r in 0 .. target.size() {
                if buffer.get(i + r) != target.get(r) {
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

pub fn contains(buffer: &dyn Array<u8>, target: &dyn Array<u8>) -> bool {
    return index_of(buffer, target).is_some();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_str_contains() {
        assert_eq!(contains(&Vector::from_slice(b"WIFI DISCONNECT"), &Vector::from_slice(b"WIFI GOT IP")), false);
        assert_eq!(contains(&Vector::from_slice(b"hello world"), &Vector::from_slice(b"wo")), true);
        assert_eq!(contains(&Vector::from_slice(b"hello world"), &Vector::from_slice(b"woldz")), false);
        assert_eq!(contains(&Vector::from_slice(b"hello world"), &Vector::from_slice(b"")), false);
        assert_eq!(contains(&Vector::from_slice(b""), &Vector::from_slice(b"woldz")), false);
        assert_eq!(contains(&Vector::from_slice(b" "), &Vector::from_slice(b"woldz")), false);
        assert_eq!(contains(&Vector::from_slice(b"     "), &Vector::from_slice(b"woldz")), false);
    }

    #[test]
    fn test_index_of() {
        assert_eq!(index_of(&Vector::from_slice(b"+CIPSTATUS:23"), &Vector::from_slice(b":")), Some(10));
        assert_eq!(index_of(&Vector::from_slice(b"+CIPSTATUS:23"), &Vector::from_slice(b"+")), Some(0));
        assert_eq!(index_of(&Vector::from_slice(b"+CIPSTATUS:23"), &Vector::from_slice(b"234")), None);
    }
}