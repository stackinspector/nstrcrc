#![no_std]

mod rev_consts;
mod rev;
pub use rev::FindContext;

pub const F64_MAX_SAFE_INT: u64 = 9007199254740991;
