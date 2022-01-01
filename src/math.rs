pub fn int_to_char(number: u8) -> u8 {
    if number > 9 {
        return b'?';
    } else {
        return number + 48;
    }
}

pub fn int_to_hex(number: u8) -> u8 {
    if number < 10 {
        return int_to_char(number);
    } else {
        return number - 10 + b'A';
    }
}

// Given a number, how many digits is it
pub fn digits(number: u32) -> u8 {
    // Count how many characters there are
    let mut digits = 0u8;
    let mut counter = number;
    while counter > 0 {
        counter /= 10;
        digits += 1;
    }
    return digits;

}

pub fn to_base(number: u32, base: u32) -> [u8; 10] {
    let result = &mut[b' '; 10];
    let mut idx = 0;
    let mut temp = number;
    while temp > 0 {
        let element = temp % base;
        temp /= base;
        result[idx] = int_to_hex(element as u8);
        idx += 1;
    }

    reverse(result);
    compact(result);
    return *result;
}

fn reverse(arr: &mut [u8]) {
    let size = arr.len();
    let mut i = 0;
    let mut r = size - 1;
    while i < size / 2 {
        let temp = arr[i];
        arr[i] = arr[r];
        arr[r] = temp;
        r -= 1;
        i+=1;
    }
}

fn compact(arr: &mut [u8]) {
    let mut left = 0;
    let mut right = 0;
    let size = arr.len();
    while right < size && left < size {
        if arr[left] == b' ' && arr[right] != b' ' {
            let temp = arr[left];
            arr[left] = arr[right];
            arr[right] = temp;
            right += 1;
            left += 1;
        } else if arr[left] == b' ' {
            right += 1;
        } else {
            left += 1;
            right += 1;
        }
    }
}

// There's a bug somewhere
// it outputs things like 0982892 on occasion
pub fn itoa_u64(val: u64) -> [u8; 20] {
    let result = &mut[b' '; 20];
    let upper_val = val / 100000000;
    let lower_val = val % 100000000;

    let upper: [u8; 10] = to_base(upper_val as u32, 10);
    let lower: [u8; 10] = to_base(lower_val as u32, 10);
    let mut i = 0;
    while i < 10 {
        result[i + 10] = lower[i];
        i += 1;
    }
    i = 0;
    while i < 10 {
        result[i] = upper[i];
        i += 1;
    }

    // Ugh, now compact it
    compact(result);
    return *result;
}

pub fn itoa_u32(val: u32) -> [u8; 10] {
    return to_base(val, 10);
}

pub fn itoa_u16(val: u16) -> [u8; 10] {
    return to_base(val as u32, 10);
}

pub fn itoa_u8(val: u8) -> [u8; 10] {
    return to_base(val as u32, 10);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_itoa() {
        assert_eq!(&itoa_u64(10345612345612345), b"10345612345612345   ");
        assert_eq!(digits(1023), 4);
        assert_eq!(int_to_char(0), b'0');
        assert_eq!(int_to_char(5), b'5');
        assert_eq!(int_to_char(8), b'8');
        assert_eq!(int_to_char(9), b'9');
        assert_eq!(&itoa_u32(19), b"19        ");
        assert_eq!(&itoa_u32(180), b"180       ");
        assert_eq!(&itoa_u32(1028191), b"1028191   ");
        assert_eq!(&itoa_u32(1220221), b"1220221   ");
        assert_eq!(&itoa_u32(1234567890), b"1234567890");
        assert_eq!(&itoa_u32(123456789), b"123456789 ");
        assert_eq!(&itoa_u32(12345678), b"12345678  ");
        assert_eq!(&itoa_u32(1234567), b"1234567   ");
        assert_eq!(&to_base(255, 16), b"FF        ");
        assert_eq!(&to_base(2700230707, 16), b"A0F24033  ");
    }
}