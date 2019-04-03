use std::io::Read;

use crate::byte_reader::ByteRead;

mod utils {
    use std::io;
    use std::io::Read;

    use crypto::aes;
    use crypto::aes::KeySize;
    use crypto::blockmodes::NoPadding;
    use crypto::buffer::{RefReadBuffer, RefWriteBuffer};
    use crypto::hmac::Hmac;
    use crypto::pbkdf2;
    use crypto::sha1::Sha1;
    use crypto::symmetriccipher::{Decryptor, SymmetricCipherError};
    use libflate::zlib::Decoder;

    pub fn pbkdf2(password: &[u8], salt: &[u8], c: u32, size: usize) -> Vec<u8> {
        let mut bytes = vec![0; size];
        let mut h = Hmac::new(Sha1::new(), password);
        pbkdf2::pbkdf2(&mut h, salt, c, &mut bytes);
        bytes
    }

    pub fn aes(key: &[u8], iv: &[u8], data: &[u8]) -> Result<Vec<u8>, SymmetricCipherError> {
        let mut cipher: Box<Decryptor> = aes::cbc_decryptor(KeySize::KeySize256, &key, &iv, NoPadding);
        let mut bytes = vec![0u8; data.len()];
        let mut read = RefReadBuffer::new(&data);
        let mut write = RefWriteBuffer::new(&mut bytes);
        cipher.decrypt(&mut read, &mut write, true)?;
        Ok(bytes)
    }

    pub fn zlib(data: &[u8]) -> io::Result<Vec<u8>> {
        let mut bytes = vec![];
        let mut decoder = Decoder::new(data)?;
        decoder.read_to_end(&mut bytes)?;
        Ok(bytes)
    }
}

fn decrypt(mut r: impl Read, password: &[u8]) -> Vec<u8> {
    let _ = r.read_be_u16();

    let _ = r.read_u8();

    let salt = r.read_u8_vec().unwrap();

    let key = utils::pbkdf2(password, &salt, 10000, 32);

    let iv = r.read_u8_vec().unwrap();

    let _ = r.read_u8_vec();

    let block = r.read_u8_vec().unwrap();
    let block = utils::aes(&key, &iv, &block).unwrap();
    let mut block = block.as_slice();

    let iv2 = block.read_u8_vec().unwrap();

    let key2 = block.read_u8_vec().unwrap();

    let _ = block.read_u8_vec();

    let block2 = r.read_u8_vec_to_end().unwrap();
    let block2 = utils::aes(&key2, &iv2, &block2).unwrap();

    utils::zlib(&block2).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use regex::Regex;

    use super::*;

    #[test]
    fn test_decrypt() {
        let file = "./samples/SafeInCloud.db";
        let password = "TheHarde5tPassw@ord!nT#3World";

        let mut file = File::open(file).unwrap();

        let result = decrypt(&mut file, password.as_bytes());

        // assert utf-8 bom
        assert_eq!(&result[..3], [0xEF, 0xBB, 0xBF]);

        let result = String::from_utf8(result[3..].to_vec()).unwrap();
        assert!(Regex::new(r"^<\?xml.*\?>\r\n<database>[\s\S]*</database>$").unwrap().is_match(&result));
    }
}