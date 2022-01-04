// Technically this supports up-to base 26 :P
pub fn int_to_hex(number: u8) -> u8 {
    if number < 10 {
        return number + 48;
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

pub fn to_base(number: u64, base: u64) -> [u8; 20] {
    let result = &mut[b' '; 20];
    let mut idx = 0;
    let mut temp = number;
    while temp >= 0 {
        let element = temp % base;
        temp /= base;
        result[idx] = int_to_hex(element as u8);
        idx += 1;
        
        if temp == 0 {
            break;
        }
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

pub fn itoa_u64(val: u64) -> [u8; 20] {
    return to_base(val, 10);
}

pub fn itoa_u32(val: u32) -> [u8; 20] {
    return to_base(val as u64, 10);
}

pub fn itoa_u16(val: u16) -> [u8; 20] {
    return to_base(val as u64, 10);
}

pub fn itoa_u8(val: u8) -> [u8; 20] {
    return to_base(val as u64, 10);
}

// Amazing prng XORSHIFT+
// https://en.wikipedia.org/wiki/Xorshift
// 128 bits is kinda overkill though.
static mut XORSHIFT_REGS: [u64;2] = [0xFAE0, 0xFFAA_FFDC];
pub fn rand() -> u64 {
    unsafe {
        let mut t = XORSHIFT_REGS[0];
        let s = XORSHIFT_REGS[1];
        XORSHIFT_REGS[0] = s;
        t ^= t << 23;
        t ^= t >> 18;
        t ^= s ^ (s >> 5);
        XORSHIFT_REGS[1] = t;
        return t + s;
    }
}

pub fn seed_rand(val: u64) {
    unsafe {
        XORSHIFT_REGS[0] = val;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_itoa() {
        assert_eq!(&itoa_u64(10345612345612345), b"10345612345612345   ");
        assert_eq!(digits(1023), 4);
        assert_eq!(int_to_hex(0), b'0');
        assert_eq!(int_to_hex(5), b'5');
        assert_eq!(int_to_hex(8), b'8');
        assert_eq!(int_to_hex(9), b'9');
        assert_eq!(&itoa_u32(19), b"19                  ");
        assert_eq!(&itoa_u32(180), b"180                 ");
        assert_eq!(&itoa_u32(1), b"1                   ");
        assert_eq!(&itoa_u32(10), b"10                  ");
        assert_eq!(&itoa_u32(101), b"101                 ");
        assert_eq!(&itoa_u32(1010), b"1010                ");
        assert_eq!(&itoa_u64(10000), b"10000               ");
        assert_eq!(&itoa_u32(3000002), b"3000002             ");
        assert_eq!(&itoa_u32(1028191), b"1028191             ");
        assert_eq!(&itoa_u32(1220221), b"1220221             ");
        assert_eq!(&itoa_u32(1234567890), b"1234567890          ");
        assert_eq!(&itoa_u32(123456789), b"123456789           ");
        assert_eq!(&itoa_u32(12345678), b"12345678            ");
        assert_eq!(&itoa_u32(1234567), b"1234567             ");
        assert_eq!(&to_base(255, 16), b"FF                  ");
        assert_eq!(&to_base(2700230707, 16), b"A0F24033            ");
    }
}