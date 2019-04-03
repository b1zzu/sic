use std::io::Read;
use std::io::Result;

pub trait ByteRead: Read {
    /// Read one byte
    fn read_u8(&mut self) -> Result<u8> {
        let mut bytes = [0; 1];
        self.read(&mut bytes)?;
        Ok(bytes[0])
    }

    /// Read two bytes and convert them in u16 using BigEndian
    fn read_be_u16(&mut self) -> Result<u16> {
        let mut bytes = [0; 2];
        self.read(&mut bytes)?;
        Ok(u16::from_be_bytes(bytes))
    }

    /// Read an array of bytes which size is determinate from the first byte
    fn read_u8_vec(&mut self) -> Result<Vec<u8>> {
        let size = self.read_u8()?;
        let mut bytes = vec![0; size as usize];
        self.read(&mut bytes)?;
        Ok(bytes)
    }

    /// Read all remaining bytes
    fn read_u8_vec_to_end(&mut self) -> Result<Vec<u8>> {
        let mut bytes = vec![];
        self.read_to_end(&mut bytes)?;
        Ok(bytes)
    }
}

impl<T: Read> ByteRead for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_u8() {
        let mut r = vec![0x0F, 0xF0];
        let mut r = r.as_slice();

        assert_eq!(r.read_u8().unwrap(), 15);
        assert_eq!(r.read_u8().unwrap(), 240);
    }

    #[test]
    fn test_read_be_u16() {
        let mut r = vec![0x0F, 0xF0, 0x12, 0xA1];
        let mut r = r.as_slice();

        assert_eq!(r.read_be_u16().unwrap(), 4080);
        assert_eq!(r.read_be_u16().unwrap(), 4769);
    }

    #[test]
    fn test_read_u8_vec() {
        let mut r = vec![0x02, 0xAA, 0xBB, 0xCC];
        let mut r = r.as_slice();

        assert_eq!(r.read_u8_vec().unwrap(), vec![0xAA, 0xBB]);
    }

    #[test]
    fn test_read_u8_vec_to_end() {
        let mut r = vec![0x02, 0xAA, 0xBB, 0xCC];
        let mut r = r.as_slice();

        r.read_u8();
        assert_eq!(r.read_u8_vec_to_end().unwrap(), vec![0xAA, 0xBB, 0xCC]);
    }
}
