use crate::datastructures::*;

// Technically this supports up-to base 26 :P
pub fn int_to_hex(number: u8) -> u8 {
    if number < 10 {
        return number + 48;
    } else {
        return number - 10 + b'A';
    }
}

pub fn min(left: usize, right: usize) -> usize {
    if left > right {
        return right;
    } else {
        return left;
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

pub fn to_base(number: u64, base: u64) -> Vector::<u8> {
    let mut result = Vector::new();
    let mut temp = number;
    loop {
        let element = temp % base;
        temp /= base;
        result.enqueue(int_to_hex(element as u8));        
        if temp == 0 {
            break;
        }
    }

    return result.reverse();
}

pub fn itoa_u64(val: u64) -> Vector::<u8> {
    return to_base(val, 10);
}

pub fn itoa_u32(val: u32) -> Vector::<u8> {
    return to_base(val as u64, 10);
}

pub fn itoa_u16(val: u16) -> Vector::<u8> {
    return to_base(val as u64, 10);
}

pub fn itoa_u8(val: u8) -> Vector::<u8> {
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
    use crate::*;
    use super::*;

    fn vecs_eq(left: Vector::<u8>, right: Vector::<u8>) {
        assert_eq!(left.size(), right.size());
        for idx in 0 .. left.size() {
            assert_eq!(left.get(idx), right.get(idx));
        }
    }

    #[test]
    fn test_itoa() {
        assert_eq!(digits(1023), 4);
        assert_eq!(int_to_hex(0), b'0');
        assert_eq!(int_to_hex(5), b'5');
        assert_eq!(int_to_hex(8), b'8');
        assert_eq!(int_to_hex(9), b'9');
        vecs_eq(itoa_u64(10345612345612345), vec_str!(b"10345612345612345"));
        vecs_eq(itoa_u64(19), vec_str!(b"19"));
        vecs_eq(itoa_u64(180), vec_str!(b"180"));
        vecs_eq(itoa_u64(1), vec_str!(b"1"));
        vecs_eq(itoa_u64(10), vec_str!(b"10"));
        vecs_eq(itoa_u64(101), vec_str!(b"101"));
        vecs_eq(itoa_u64(1010), vec_str!(b"1010"));
        vecs_eq(itoa_u64(10000), vec_str!(b"10000"));
        vecs_eq(itoa_u64(3000002), vec_str!(b"3000002"));
        vecs_eq(itoa_u64(1028191), vec_str!(b"1028191"));
        vecs_eq(itoa_u64(1220221), vec_str!(b"1220221"));
        vecs_eq(itoa_u64(1234567890), vec_str!(b"1234567890"));
        vecs_eq(itoa_u64(123456789), vec_str!(b"123456789"));
        vecs_eq(itoa_u64(12345678), vec_str!(b"12345678"));
        vecs_eq(itoa_u64(1234567), vec_str!(b"1234567"));
        vecs_eq(to_base(255, 16), vec_str!(b"FF"));
        vecs_eq(to_base(2700230707, 16), vec_str!(b"A0F24033"));
    }
}