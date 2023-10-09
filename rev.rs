use libc::{printf, sprintf, scanf, sscanf};
use crate::rev_consts::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CRC {
    pub crc: u32,
    pub index: u8,
}
#[no_mangle]
pub unsafe extern "C" fn crc32(str: *const u8) -> CRC {
    let mut result = CRC {
        crc: 0xffffffff,
        index: 0xff,
    };
    let mut i: *const u8 = str;
    while *i != 0 {
        result.index = (result.crc & 0xff as u32 ^ *i as u32) as u8;
        result.crc = result.crc >> 8 ^ TABLE[result.index as usize];
        i = i.offset(core::mem::size_of::<u8>() as isize);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn check(high: u64, indexes: *mut u8) -> i16 {
    let mut buf: [u8; 17] = [0; 17];
    sprintf(
        buf.as_mut_ptr() as *mut i8,
        b"%ju\0" as *const u8 as *const i8,
        high,
    );
    let result: CRC = crc32(buf.as_mut_ptr() as *const u8);
    let index: u8 = result.index;
    if index != *indexes.offset(3) {
        return -1;
    }
    let mut crc: u32 = result.crc;
    let mut low: i16 = 0;
    #[allow(unused_assignments)]
    let mut num: u8 = 0;
    let mut i: i8 = 2;
    while i > -1 {
        num = (crc & 0xff as u32 ^ *indexes.offset(i as isize) as u32).wrapping_sub(48) as u8;
        #[allow(unused_comparisons)]
        if !(0 <= num && num < 10) {
            return -1;
        }
        low = (low as f64 + num as f64 * 10f64.powf(i as f64)) as i16;
        crc = TABLE[*indexes.offset(i as isize) as usize] ^ crc >> 8;
        i -= 1;
    }
    return low;
}
#[no_mangle]
pub unsafe extern "C" fn crack(mut crc: u32) -> u64 {
    let mut indexes: [u8; 4] = [0; 4];
    crc ^= 0xffffffff;
    let mut i: u16 = 1;
    while (i) < 1000 {
        let mut buf: [u8; 4] = [0; 4];
        sprintf(
            buf.as_mut_ptr() as *mut i8,
            b"%u\0" as *const u8 as *const i8,
            i as u32,
        );
        if crc == (crc32(buf.as_mut_ptr() as *const u8)).crc {
            return i as u64;
        }
        i = i.wrapping_add(1);
    }
    let mut i_0: i8 = 3;
    while i_0 > -1 {
        indexes[(3 - i_0) as usize] = LAST_INDEX[(crc >> (i_0 << 3)) as usize] as u8;
        crc ^= TABLE[indexes[(3 - i_0) as usize] as usize] >> ((3 - i_0) << 3);
        i_0 -= 1;
    }
    #[allow(unused_assignments)]
    let mut low: i16 = 0;
    let mut i_1: u64 = 0;
    loop {
        i_1 = i_1.wrapping_add(1);
        low = check(i_1, indexes.as_mut_ptr());
        if low >= 0 {
            return i_1.wrapping_mul(1000).wrapping_add(low as u64);
        }
        if !(i_1 < u64::MAX.wrapping_div(1000)) {
            break;
        }
    }
    return 0;
}
pub unsafe extern "C" fn main_0(args: i32, argv: *mut *mut i8) -> i32 {
    let mut crc: u32 = 0;
    if args > 1 {
        sscanf(
            *argv.offset(1),
            b"%8x\0" as *const u8 as *const i8,
            &mut crc as *mut u32,
        );
    } else if scanf(b"%8x\0" as *const u8 as *const i8, &mut crc as *mut u32) != 1 {
        return -1;
    }
    printf(b"%ju\n\0" as *const u8 as *const i8, crack(crc));
    return 0;
}
