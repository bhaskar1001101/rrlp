//! RLP decoding functionality.

use crate::{Error, MAX_LENGTH};

/// Core trait for decoding RLP data into a type
pub trait Decode: Sized {
    /// Decode RLP bytes into this type
    fn decode_from(buf: &mut Vec<u8>) -> Result<Self, Error>;
}

/// Decode RLP bytes into a target type
// pub fn decode<T: Decode + ?Sized>(buf: &[u8]) -> Result<T, Error> {
//     if buf.is_empty() {
//         return Err(Error::InputTooShort);
//     }

//     // let output = Vec::new();

//     T::decode_from(&mut Vec::from(buf))
// }

// When I decode I will be taking input from
// Vec<u8> for now.
// String (In Hex Form)
// &[u8] (Fixed Size)
// Write derive macros! for Encode, Decode

// def rlp_decode(input):
//     if len(input) == 0:
//         return
//     output = ''
//     (offset, dataLen, type) = decode_length(input)
//     if type is str:
//         output = instantiate_str(substr(input, offset, dataLen))
//     elif type is list:
//         output = instantiate_list(substr(input, offset, dataLen))
//     output += rlp_decode(substr(input, offset + dataLen))
//     return output

// def decode_length(input):
//     length = len(input)
//     if length == 0:
//         raise Exception("input is null")
//     prefix = ord(input[0])
//     if prefix <= 0x7f:
//         return (0, 1, str)
//     elif prefix <= 0xb7 and length > prefix - 0x80:
//         strLen = prefix - 0x80
//         return (1, strLen, str)
//     elif prefix <= 0xbf and length > prefix - 0xb7 and length > prefix - 0xb7 + to_integer(substr(input, 1, prefix - 0xb7)):
//         lenOfStrLen = prefix - 0xb7
//         strLen = to_integer(substr(input, 1, lenOfStrLen))
//         return (1 + lenOfStrLen, strLen, str)
//     elif prefix <= 0xf7 and length > prefix - 0xc0:
//         listLen = prefix - 0xc0;
//         return (1, listLen, list)
//     elif prefix <= 0xff and length > prefix - 0xf7 and length > prefix - 0xf7 + to_integer(substr(input, 1, prefix - 0xf7)):
//         lenOfListLen = prefix - 0xf7
//         listLen = to_integer(substr(input, 1, lenOfListLen))
//         return (1 + lenOfListLen, listLen, list)
//     raise Exception("input does not conform to RLP encoding form")

// def to_integer(b):
//     length = len(b)
//     if length == 0:
//         raise Exception("input is null")
//     elif length == 1:
//         return ord(b[0])
//     return ord(substr(b, -1)) + to_integer(substr(b, 0, -1)) * 256


impl Decode for String {
    fn decode_from(buf: &mut Vec<u8>) -> Result<Self, Error> {
        Ok(String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_single_byte() {
        assert_eq!(decode(&"0x00".as_bytes()).unwrap(), vec![0x00]);
        assert_eq!(decode(&"0x7f".as_bytes()).unwrap(), vec![0x7f]);
        assert_eq!(decode(&"0x80".as_bytes()).unwrap(), vec![0]);
        assert_eq!(decode(&"0x81ff".as_bytes()).unwrap(), vec![0xff]);
        // assert_eq!(u8::decode(&[0x00]).unwrap(), 0x00);
        // assert_eq!(u8::decode(&[0x7f]).unwrap(), 0x7f);
        // assert_eq!(u8::decode(&[0x80]).unwrap(), 0);
        // assert_eq!(u8::decode(&[0x81, 0xff]).unwrap(), 0xff);
    }

    // #[test]
    // fn decode_string() {

    // }

    // #[test]
    // fn decode_list() {
    //     // Empty list
    //     let empty: Vec<u8> = Vec::decode(&[0xc0]).unwrap();
    //     assert!(empty.is_empty());

    //     // // List with single item
    //     let single = Vec::<u8>::decode(&[0xc1, 0x00]).unwrap();
    //     assert_eq!(single, vec![0]);

    //     // // List with multiple items
    //     let data = vec![0xc8, 0x83, b'c', b'a', b't', 0x83, b'd', b'o', b'g'];
    //     let decoded: Vec<String> = decode(&data).unwrap();
    //     assert_eq!(decoded, vec!["cat".to_string(), "dog".to_string()]);
    // }
}