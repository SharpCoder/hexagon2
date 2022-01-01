pub fn math_int_to_char(number: u8) -> u8 {
    if number > 9 {
        return b'?';
    } else {
        return number + 48;
    }
}

// Given a number, how many digits is it
pub fn math_digits(number: u64) -> u8 {
    // Count how many characters there are
    let mut digits = 0u8;
    let mut counter = number;
    while counter > 0 {
        counter /= 10;
        digits += 1;
    }
    return digits;

}

// Takes a number and returns the character
// representation of this number.
pub fn itoa_u<'a> (number: u64) -> [u8; 10] {
    // This is definitely a no-hire interview response :P

    // For now, we're using static length stuff
    // this is debug anyway.
    let mut result = &mut [b' '; 10];
    let mut idx = 0;
    let mut temp = number;
    while temp > 0 {
        let mut element = temp % 10;
        temp /= 10;
        result[idx] = math_int_to_char(element as u8);
        idx += 1;
    }

    // Reverse
    let stop = (idx / 2) - 1;
    idx -= 1;
    let mut r = 0;
    while idx > stop {
        let temp = result[r];
        result[r] = result[idx];
        result[idx] = temp;
        r += 1;
        idx -= 1;
    }


    return *result;
}

pub fn itoa_u64(val: u64) -> [u8; 10] {
    return itoa_u(val);
}

pub fn itoa_u32(val: u32) -> [u8; 10] {
    return itoa_u(val as u64);
}

pub fn itoa_u16(val: u16) -> [u8; 10] {
    return itoa_u(val as u64);
}

pub fn itoa_u8(val: u8) -> [u8; 10] {
    return itoa_u(val as u64);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_itoa() {
        assert_eq!(math_digits(1023), 4);
        assert_eq!(math_int_to_char(0), b'0');
        assert_eq!(math_int_to_char(5), b'5');
        assert_eq!(math_int_to_char(8), b'8');
        assert_eq!(math_int_to_char(9), b'9');
        assert_eq!(&itoa_u64(180), b"180       ");
        assert_eq!(&itoa_u64(1028191), b"1028191   ");
        assert_eq!(&itoa_u64(1220221), b"1220221   ");
        assert_eq!(&itoa_u64(1234567890), b"1234567890");
        assert_eq!(&itoa_u64(123456789), b"123456789 ");
        assert_eq!(&itoa_u64(12345678), b"12345678  ");
        assert_eq!(&itoa_u64(1234567), b"1234567   ");
    }
}