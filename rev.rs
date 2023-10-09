use crate::rev_consts::*;

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

    let mut i: i8 = 2;
    while i > -1 {
        let num = ((crc & 0xff ^ indexes[i as usize] as u32) - 48) as u8;
        if !(num < 10) {
            return -1;
        }
        low += (num as i16) * 10i16.pow(i as u32);
        crc = TABLE[indexes[i as usize] as usize] ^ crc >> 8;
        i -= 1;
    }

    low
}

pub fn find(mut crc: u32) -> Vec<u64> {
    let mut results = Vec::new();
    let mut indexes = [0; 4];
    crc ^= u32::MAX;

    let mut i: u16 = 1;
    while (i) < 1000 {
        if crc == crc32(i.to_string().as_bytes()).crc {
            results.push(i as u64);
        }
        i += 1;
    }

    let mut i: i8 = 3;
    while i > -1 {
        indexes[(3 - i) as usize] = LAST_INDEX[(crc >> (i << 3)) as usize];
        crc ^= TABLE[indexes[(3 - i) as usize] as usize] >> ((3 - i) << 3);
        i -= 1;
    }

    let mut i: u64 = 0;
    loop {
        i += 1;
        let low = check(i, &mut indexes);
        if low >= 0 {
            results.push(i * 1000 + (low as u64));
        }
        if !(i < (u64::MAX / 1000)) {
            break;
        }
    }

    results
}
