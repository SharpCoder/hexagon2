pub fn contains(buffer: &[u8], target: &[u8]) -> bool {
    if target.len() == 0 {
        return true;
    } else if buffer.len() == 0 {
        return false;
    } else if buffer.len() < target.len() {
        return false;
    }

    for i in 0 .. buffer.len() - target.len() {
        if buffer[i] == target[0] {
            let mut found = true;
            for r in 0 .. target.len() {
                if buffer[i + r] != target[r] {
                    found = false;
                    break;
                }
            }

            if found {
                return true;
            }
        }
    }
    return false;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_str_contains() {
        assert_eq!(contains(b"WIFI DISCONNECT", b"WIFI GOT IP"), false);
        assert_eq!(contains(b"hello world", b"wo"), true);
        assert_eq!(contains(b"hello world", b"woldz"), false);
        assert_eq!(contains(b"hello world", b""), true);
        assert_eq!(contains(b"", b"woldz"), false);
        assert_eq!(contains(b" ", b"woldz"), false);
        assert_eq!(contains(b"     ", b"woldz"), false);
    }
}