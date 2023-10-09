fn main() {
    let crchexstr = std::env::args().nth(1).unwrap();
    let crcbytes = hex::decode(crchexstr).unwrap();
    let crc = u32::from_be_bytes(crcbytes.try_into().unwrap());
    println!("{:?}", uidcrc::find(crc));
}
