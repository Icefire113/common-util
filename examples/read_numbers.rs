use common_util::bin_read::read_u8_le;
use common_util::bin_read::read_u16_le;
use common_util::bin_read::read_u32_le;

fn main() {
    // Any Read source works here a byte slice.
    let mut data = [0x78u8, 0x56, 0x34, 0x12, 0xAB, 0xCD, 0x01].as_slice();

    let b = read_u8_le(&mut data).unwrap();
    let n16 = read_u16_le(&mut data).unwrap();
    let n32 = read_u32_le(&mut data).unwrap();

    println!("u8  = 0x{b:02x}");
    println!("u16 = 0x{n16:04x}");
    println!("u32 = 0x{n32:08x}");
}
