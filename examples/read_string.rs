use common_util::bin_read::read_string;
use common_util::bin_read::read_u32_le;

fn main() {
    let mut data = b"hello\0\x78\x56\x34\x12".as_slice();

    let s = read_string(&mut data).unwrap();
    println!("string: {s}");

    let n = read_u32_le(&mut data).unwrap();
    println!("u32: 0x{n:08x}");
}
