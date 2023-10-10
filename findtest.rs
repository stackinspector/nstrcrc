fn main() {
    let crchexstr = std::env::args().nth(1).unwrap();
    let crcbytes = hex::decode(crchexstr).unwrap();
    let crc = u32::from_be_bytes(crcbytes.try_into().unwrap());
    let ctx = uidcrc::FindContext::new(crc, u32::MAX as u64);
    for val in ctx {
        println!("{}", val);
    }
}
