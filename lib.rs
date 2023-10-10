#![no_std]

mod consts;
use consts::*;

pub struct FindContext {
    crc: u32,
    max: u64,
    buf: itoa::Buffer,
    last: u64,
    indexes: Option<[u8; 4]>,
}

impl FindContext {
    pub fn new(mut crc: u32, max: u64) -> FindContext {
        crc ^= u32::MAX;
        FindContext {
            crc,
            max,
            buf: itoa::Buffer::new(),
            last: 1,
            indexes: None,
        }
    }

    fn crc32(&mut self, val: u64) -> (u32, u8) {
        let s = self.buf.format(val).as_bytes();
        let mut crc = u32::MAX;
        let mut index = u8::MAX;
        for b in s {
            index = (crc & 0xff ^ *b as u32) as u8;
            crc = crc >> 8 ^ TABLE[index as usize];
        }
        (crc, index)
    }

    fn switch(&mut self) {
        self.last = 1;
        let mut indexes = [0; 4];
        for i in 0..indexes.len() {
            indexes[i] = LAST_INDEX[(self.crc >> ((3 - i) << 3)) as usize];
            self.crc ^= TABLE[indexes[i] as usize] >> (i << 3);
        }
        let _indexes = core::mem::replace(&mut self.indexes, Some(indexes));
        assert_eq!(_indexes, None);
    }
}

impl Iterator for FindContext {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        match self.indexes {
            None => {
                for val in self.last..1000 {
                    if self.crc == self.crc32(val).0 {
                        self.last = val + 1;
                        return Some(val);
                    }
                }
                self.switch();
                self.next()
            }
            Some(indexes) => {
                'a: for high in self.last..(self.max / 1000) {
                    let (mut crc, index) = self.crc32(high);
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
                    self.last = high + 1;
                    let val = high * 1000 + (low as u64);
                    return Some(val);
                }
                None
            }
        }
    }
}
