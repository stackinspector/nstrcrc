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
    for b in s {
        index = (crc & 0xff ^ *b as u32) as u8;
        crc = crc >> 8 ^ TABLE[index as usize];
    }
    CRC { crc, index }
}

fn check(high: u64, indexes: &mut [u8; 4]) -> Option<u16> {
    let CRC { mut crc, index } = crc32(high.to_string().as_bytes());
    if index != indexes[3] {
        return None;
    }

    // vaild: 0..999
    let mut low = 0;
    for i in (0..=2).rev() {
        let num = (crc & 0xff ^ indexes[i] as u32).wrapping_sub(48) as u8;
        if !(num < 10) {
            return None;
        }
        low += (num as u16) * 10u16.pow(i as u32);
        crc = TABLE[indexes[i] as usize] ^ crc >> 8;
    }
    Some(low)
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

    for high in 1..(MAX_BOUND / 1000) {
        if let Some(low) = check(high, &mut indexes) {
            results.push(high * 1000 + (low as u64));
        }
    }

    results
}
