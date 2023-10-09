#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// #![register_tool(c2rust)]
// #![feature(register_tool)]
extern "C" {
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type int8_t = libc::c_schar;
pub type int16_t = libc::c_short;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type uintmax_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crc_t {
    pub crc: uint32_t,
    pub index: uint8_t,
}
static mut table: [uint32_t; 256] = crate::rev_consts::TABLE;
static mut last_index: [uint32_t; 256] = crate::rev_consts::LAST_INDEX;
#[no_mangle]
pub unsafe extern "C" fn crc32(mut str: *const libc::c_uchar) -> crc_t {
    let mut result: crc_t = {
        let mut init = crc_t {
            crc: 0xffffffff as libc::c_uint,
            index: 0xff as libc::c_int as uint8_t,
        };
        init
    };
    let mut i: *const libc::c_uchar = str;
    while *i as libc::c_int != 0 as libc::c_int {
        result
            .index = (result.crc & 0xff as libc::c_int as libc::c_uint
            ^ *i as libc::c_uint) as uint8_t;
        result.crc = result.crc >> 8 as libc::c_int ^ table[result.index as usize];
        i = i.offset(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong as isize);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn check(
    mut high: uintmax_t,
    mut indexes: *mut uint8_t,
) -> int16_t {
    let mut buf: [libc::c_uchar; 17] = [
        0 as libc::c_int as libc::c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    sprintf(
        buf.as_mut_ptr() as *mut libc::c_char,
        b"%ju\0" as *const u8 as *const libc::c_char,
        high,
    );
    let mut result: crc_t = crc32(buf.as_mut_ptr() as *const libc::c_uchar);
    let mut index: uint8_t = result.index;
    if index as libc::c_int != *indexes.offset(3 as libc::c_int as isize) as libc::c_int
    {
        return -(1 as libc::c_int) as int16_t;
    }
    let mut crc: uint32_t = result.crc;
    let mut low: int16_t = 0 as libc::c_int as int16_t;
    let mut num: uint8_t = 0;
    let mut i: int8_t = 2 as libc::c_int as int8_t;
    while i as libc::c_int > -(1 as libc::c_int) {
        num = (crc & 0xff as libc::c_int as libc::c_uint
            ^ *indexes.offset(i as isize) as libc::c_uint)
            .wrapping_sub(48 as libc::c_int as libc::c_uint) as uint8_t;
        if !(0 as libc::c_int <= num as libc::c_int
            && (num as libc::c_int) < 10 as libc::c_int)
        {
            return -(1 as libc::c_int) as int16_t;
        }
        low = (low as libc::c_double
            + num as libc::c_int as libc::c_double
                * pow(10 as libc::c_int as libc::c_double, i as libc::c_double))
            as int16_t;
        crc = table[*indexes.offset(i as isize) as usize] ^ crc >> 8 as libc::c_int;
        i -= 1;
    }
    return low;
}
#[no_mangle]
pub unsafe extern "C" fn crack(mut crc: uint32_t) -> uintmax_t {
    let mut indexes: [uint8_t; 4] = [0; 4];
    crc ^= 0xffffffff as libc::c_uint;
    let mut i: uint16_t = 1 as libc::c_int as uint16_t;
    while (i as libc::c_int) < 1000 as libc::c_int {
        let mut buf: [libc::c_uchar; 4] = [0 as libc::c_int as libc::c_uchar, 0, 0, 0];
        sprintf(
            buf.as_mut_ptr() as *mut libc::c_char,
            b"%u\0" as *const u8 as *const libc::c_char,
            i as libc::c_int,
        );
        if crc == (crc32(buf.as_mut_ptr() as *const libc::c_uchar)).crc {
            return i as uintmax_t;
        }
        i = i.wrapping_add(1);
    }
    let mut i_0: int8_t = 3 as libc::c_int as int8_t;
    while i_0 as libc::c_int > -(1 as libc::c_int) {
        indexes[(3 as libc::c_int - i_0 as libc::c_int)
            as usize] = last_index[(crc >> ((i_0 as libc::c_int) << 3 as libc::c_int))
            as usize] as uint8_t;
        crc
            ^= table[indexes[(3 as libc::c_int - i_0 as libc::c_int) as usize] as usize]
                >> ((3 as libc::c_int - i_0 as libc::c_int) << 3 as libc::c_int);
        i_0 -= 1;
    }
    let mut low: int16_t = 0;
    let mut i_1: uintmax_t = 0 as libc::c_int as uintmax_t;
    loop {
        i_1 = i_1.wrapping_add(1);
        low = check(i_1, indexes.as_mut_ptr());
        if low as libc::c_int >= 0 as libc::c_int {
            return i_1
                .wrapping_mul(1000 as libc::c_int as libc::c_ulong)
                .wrapping_add(low as libc::c_ulong);
        }
        if !(i_1
            < (18446744073709551615 as libc::c_ulong)
                .wrapping_div(1000 as libc::c_int as libc::c_ulong))
        {
            break;
        }
    }
    return 0 as libc::c_int as uintmax_t;
}
unsafe fn main_0(
    mut args: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut crc: uint32_t = 0;
    if args > 1 as libc::c_int {
        sscanf(
            *argv.offset(1 as libc::c_int as isize),
            b"%8x\0" as *const u8 as *const libc::c_char,
            &mut crc as *mut uint32_t,
        );
    } else if scanf(
        b"%8x\0" as *const u8 as *const libc::c_char,
        &mut crc as *mut uint32_t,
    ) != 1 as libc::c_int
    {
        return -(1 as libc::c_int)
    }
    printf(b"%ju\n\0" as *const u8 as *const libc::c_char, crack(crc));
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
