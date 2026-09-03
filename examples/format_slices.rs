use common_util::formatting::BinaryHexSlice;
use common_util::formatting::LowerCaseHexSlice;
use common_util::formatting::UpperCaseHexSlice;

fn main() {
    let bytes = [0xDE, 0xAD, 0xBE, 0xEF];

    println!("lower: {}", LowerCaseHexSlice(&bytes));
    println!("upper: {}", UpperCaseHexSlice(&bytes));
    println!("bin:   {}", BinaryHexSlice(&bytes));
}
