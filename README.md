# common-util

Just some common utilites that I have found a repeated need for in the past,
just consolidated into one crate so I can stop copy-pasting the same couple files everywhere.

## Features

- `bin_read`: binary reading helpers (`read_u32_le`, `read_string`, `skip`, ...) plus the `UtilReadError` type. **Default.**
- `formatting`: `Display` wrappers for formatting byte slices as hex/binary (`LowerCaseHexSlice`, `UpperCaseHexSlice`, `BinaryHexSlice`). **Default.**

## Usage

```toml
[dependencies]
bin-read-util = { version = "0.1" }
```

```rust
use bin_read_util::bin_read::read_u32_le;

let mut data = &[0x78, 0x56, 0x34, 0x12][..];
let n = read_u32_le(&mut data)?; // 0x12345678
```

## Examples

Run the bundled examples with `cargo run --example <name>`:

- `read_numbers`: reading `u8`/`u16`/`u32` from a byte slice
- `read_string`: reading a null-terminated C string
- `format_slices`: formatting a byte slice as hex/binary
