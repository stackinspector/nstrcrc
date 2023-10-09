use crate::rev_consts::*;

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

pub fn find(mut crc: u32, max: u64) -> Vec<u64> {
    let mut results = Vec::new();
    let mut indexes = [0; 4];
    crc ^= u32::MAX;

    for val in 1..1000 {
        if crc == crc32(val.to_string().as_bytes()).crc {
            results.push(dbg!(val));
        }
    }

    for i in 0..indexes.len() {
        indexes[i] = LAST_INDEX[(crc >> ((3 - i) << 3)) as usize];
        crc ^= TABLE[indexes[i] as usize] >> (i << 3);
    }

    'a: for high in 1..(max / 1000) {
        let CRC { mut crc, index } = crc32(high.to_string().as_bytes());
        if index != indexes[3] {
            continue 'a;
        }

        // vaild: 0..999
        let mut low = 0;
        for i in (0..3).rev() {
            let num = (crc & 0xff ^ indexes[i] as u32).wrapping_sub(48) as u8;
            if !(num < 10) {
                continue 'a;
            }
            low += (num as u16) * 10u16.pow(i as u32);
            crc = TABLE[indexes[i] as usize] ^ crc >> 8;
        }

        let val = high * 1000 + (low as u64);
        results.push(dbg!(val));
    }

    results
}
