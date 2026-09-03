use std::fmt::Display;

pub struct LowerCaseHexSlice<'a>(&'a [u8]);
impl Display for LowerCaseHexSlice<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in self.0 {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}

pub struct UpperCaseHexSlice<'a>(&'a [u8]);
impl Display for UpperCaseHexSlice<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in self.0 {
            write!(f, "{:02X}", b)?;
        }
        Ok(())
    }
}

pub struct BinaryHexSlice<'a>(&'a [u8]);
impl Display for BinaryHexSlice<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in self.0 {
            write!(f, "{:b}", b)?;
        }
        Ok(())
    }
}
