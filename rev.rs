use crate::rev_consts::*;

// TODO: index usize?

const SMALL_BOUND: u64 = 1000;
const MAX_BOUND: u64 = u32::MAX as u64;

struct CRC {
    crc: u32,
    index: u8,
}

fn crc32(s: &[u8]) -> CRC {
    let mut crc = u32::MAX;
    let mut index = u8::MAX;
    for i in s {
        index = (crc & 0xff ^ *i as u32) as u8;
        crc = crc >> 8 ^ TABLE[index as usize];
    }
    CRC { crc, index }
}

fn check(high: u64, indexes: &mut [u8; 4]) -> i16 {
    let CRC { mut crc, index } = crc32(high.to_string().as_bytes());
    if index != indexes[3] {
        return -1;
    }

    let mut low: i16 = 0;
    for i in (0..=2).rev() {
        let num = (crc & 0xff ^ indexes[i] as u32).wrapping_sub(48) as u8;
        if !(num < 10) {
            return -1;
        }
        low += (num as i16) * 10i16.pow(i as u32);
        crc = TABLE[indexes[i] as usize] ^ crc >> 8;
    }
    low
}

pub fn find(mut crc: u32) -> Vec<u64> {
    let mut results = Vec::new();
    let mut indexes = [0; 4];
    crc ^= u32::MAX;

    for val in 1..SMALL_BOUND {
        if crc == crc32(val.to_string().as_bytes()).crc {
            results.push(val as u64);
        }
    }

    for i in 0..indexes.len() {
        indexes[i] = LAST_INDEX[(crc >> ((3 - i) << 3)) as usize];
        crc ^= TABLE[indexes[i] as usize] >> (i << 3);
    }

    for val in 1..MAX_BOUND {
        let low = check(val, &mut indexes);
        if low >= 0 {
            results.push(val * 1000 + (low as u64));
        }
    }

    results
}
