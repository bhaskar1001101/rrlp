use crate::{Error, MAX_LENGTH};

/// Trait for types that can be RLP encoded
pub trait Encode {
    /// Append RLP encoded bytes to the provided buffer
    fn encode_to(&self, out: &mut Vec<u8>);
}

/// Encode a value into RLP bytes 
pub fn encode<T: Encode + ?Sized, O: From<Vec<u8>>>(value: &T) -> O {
    let mut out = Vec::new();
    value.encode_to(&mut out);
    out.into()
}

// fn length_of_length(len: usize) -> usize;

// fn write_length(len: usize, offset: u8, out: &mut Vec<u8>);

impl Encode for u8 {
    // https://ethereum.org/en/developers/docs/data-structures-and-encoding/rlp/
    // For a positive integer, it is converted to the shortest
    // byte array whose big-endian interpretation is the 
    // integer, and then encoded as a string according 
    // to the rules below.
    fn encode_to(&self, out: &mut Vec<u8>) {
        if *self == 0 {
            out.push(0x80 as u8);
        }
        else if *self <= 0x7f { 
            out.push(*self);
        } else {
            out.extend_from_slice(&[0x81, *self]);
        }
    }
}

// impl Encode for u16 {
//     fn encode_to(&self, out: &mut Vec<u8>);
// }

// impl Encode for u32 {
//     fn encode_to(&self, out: &mut Vec<u8>);
// }

// impl Encode for u64 {
//     fn encode_to(&self, out: &mut Vec<u8>);
// }

// impl<T: Encode> Encode for Vec<T> {
//     fn encode_to(&self, out: &mut Vec<u8>);
// }

// impl<T: Encode> Encode for &[T] {
//     fn encode_to(&self, out: &mut Vec<u8>);
// }

impl Encode for [u8] {
    fn encode_to(&self, out: &mut Vec<u8>) {
        let len = self.len();
       
       if len == 0 {
           // Empty byte array encodes as 0x80
           out.push(0x80);
        } else if len == 1 && self[0] < 0x80 {
            // Single byte < 0x80 encodes as itself
            out.push(self[0]);
       } else if len < 56 {
            // Short string: 0x80 + length followed by string
            out.push(0x80 + len as u8);
            out.extend_from_slice(self);
       } else {
            // Long string: 0xb7 + length of length,
            let len_bytes = len.to_be_bytes();
            // Trim leading zeros
            let len_bytes = &len_bytes[len_bytes.iter().position(|&x| x != 0).unwrap_or(len_bytes.len() - 1)..];
            
            // Push prefix: 0xb7 + length of the length bytes
            out.push(0xb7 + len_bytes.len() as u8);
            // Push the length bytes directly
            out.extend_from_slice(len_bytes);
            // Push the actual data
            out.extend_from_slice(self);
       }
   }
}

impl<const N: usize> Encode for [u8; N] {
    fn encode_to(&self, out: &mut Vec<u8>) {
        self.as_slice().encode_to(out) 
    }
}

impl Encode for str {
    fn encode_to(&self, out: &mut Vec<u8>) {
        self.as_bytes().encode_to(out)
    }
}

impl Encode for &str {
    fn encode_to(&self, out: &mut Vec<u8>) {
        self.as_bytes().encode_to(out)
    }
}


impl Encode for String {
    fn encode_to(&self, out: &mut Vec<u8>) {
        self.as_str().encode_to(out)
    }
}

impl<T: Encode> Encode for Vec<T> {
    fn encode_to(&self, out: &mut Vec<u8>) {
        // Oh no. More allocations.
        let mut payload = Vec::new();
        for item in self {
            item.encode_to(&mut payload);
        }
        
        let len = payload.len();
        if len < 56 {
            // Short list: 0xc0 + length followed by payload
            out.push(0xc0 + len as u8);
            out.extend_from_slice(&payload);
        } else {
            // Long list: 0xf7 + length of length,
            let len_bytes = len.to_be_bytes();
            // Trim leading zeros. How does this work.
            let len_bytes = &len_bytes[len_bytes.iter().position(|&x| x != 0).unwrap_or(len_bytes.len() - 1)..];
            
            // Push prefix: 0xf7 + length of the length bytes
            out.push(0xf7 + len_bytes.len() as u8);
            // Push the length bytes
            out.extend_from_slice(len_bytes);
            // Push the payload
            out.extend_from_slice(&payload);
        }
    }
}

#[cfg(test)]#[cfg(test)]
mod tests {
    use super::*;

    // Single byte tests (0x00-0x7f)
    #[test]
    fn test_single_bytes() {
        // Zero encodes to 0x80
        let result: Box<[u8]> = encode(&0u8);
        assert_eq!(*result, vec![0x80]);
        
        // Values < 0x80 encode as themselves
        let result: &[u8] = encode(&0x7fu8);
        assert_eq!(result, vec![0x7f]);
        let result: Vec<u8> = encode(&0x10u8);
        assert_eq!(result, vec![0x10]);
        
        // Values >= 0x80 encode as 0x81 + value
        let result: Vec<u8> = encode(&0x80u8);
        assert_eq!(result, vec![0x81, 0x80]);
        let result: Vec<u8> = encode(&0xffu8);
        assert_eq!(result, vec![0x81, 0xff]);
    }

    // Short string tests (length < 56 bytes)
    #[test]
    fn test_short_strings() {
        // Regular ASCII strings
        let result: Vec<u8> = encode("dog");
        assert_eq!(result, vec![0x83, b'd', b'o', b'g']);
        let result: Vec<u8> = encode(&String::from("dog"));
        assert_eq!(result, vec![0x83, b'd', b'o', b'g']);
        
        // UTF-8 strings
        let result: Vec<u8> = encode("中文");
        assert_eq!(result, vec![0x86, 0xe4, 0xb8, 0xad, 0xe6, 0x96, 0x87]);
        
        // String at boundary (55 bytes)
        let s = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabc";
        let mut expected = vec![0x80 + s.len() as u8];
        expected.extend_from_slice(s.as_bytes());
        let result: Vec<u8> = encode(s);
        assert_eq!(result, expected);
    }

    // Long string tests (length >= 56 bytes)
    #[test]
    fn test_long_strings() {
        // 56 bytes (minimum long string)
        let s = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcd";
        let mut expected = vec![0xb8, 0x38];  // 0xb8 + len of len (1) + len (56/0x38)
        expected.extend_from_slice(s.as_bytes());
        let result: Vec<u8> = encode(s);
        assert_eq!(result, expected);
        
        // Much longer string (1024 bytes)
        let s = "a".repeat(1024);
        let mut expected = vec![0xb9, 0x04, 0x00];  // 0xb9 + len of len (2) + len (1024)
        expected.extend_from_slice(s.as_bytes());
        let result: Vec<u8> = encode(&s);
        assert_eq!(result, expected);
    }

    // Short list tests (total payload < 56 bytes)
    #[test]
    fn test_short_lists() {
        // Empty list
        let empty: Vec<u8> = vec![];
        let result: Vec<u8> = encode(&empty);
        assert_eq!(result, vec![0xc0]);

        // This looks weird
        let empty: Vec<Vec<u8>> = vec![];
        let result: Vec<u8> = encode(&empty);
        assert_eq!(result, vec![0xc0]);

        // Single element lists
        let single = vec!["a"];  
        let result: Vec<u8> = encode(&single);
        assert_eq!(result, vec![0xc1, 0x61]);

        // Single empty string in list
        let empty_str = vec![""];
        let result: Vec<u8> = encode(&empty_str);
        assert_eq!(result, vec![0xc1, 0x80]);

        // Three short strings
        let v = vec!["cat", "dog", "pig"];
        let result: Vec<u8> = encode(&v);
        assert_eq!(result, vec![0xcc, 0x83, b'c', b'a', b't', 0x83, b'd', b'o', b'g', 0x83, b'p', b'i', b'g']);
    }

    // Long list tests (total payload >= 56 bytes)
    #[test]
    fn test_long_lists() {
        // Create a list with 56 single byte elements (zero)
        let v: Vec<u8> = vec![0; 56];
        let mut expected = vec![0xf8, 56]; // 0xf8 + len of len (1) + len (56)
        expected.extend(vec![0x80; 56]);
        let result: Vec<u8> = encode(&v);
        assert_eq!(result, expected);

        let long_string = "Lorem ipsum dolor sit amet, consectetur adipisicing elit";
        let expected = hex::decode("b8384c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e7365637465747572206164697069736963696e6720656c6974").unwrap();
        let result: Vec<u8> = encode(long_string);
        assert_eq!(result, expected);
        
        // List of strings making > 55 bytes
        let v = vec!["cat".to_string(); 20]; // Each "cat" takes 4 bytes in RLP
        let output: Vec<u8> = encode(&v);
        assert_eq!(output[0], 0xf8); // Long list marker
        assert!(output.len() > 55); // Verify it's actually a long list
        
        // Deeply nested lists
        // let mut v = vec![vec![1u8]];
        // for _ in 0..20 {
        //     v = vec![v];
        // }
        // let output: Vec<u8> = encode(&v);
        // assert_eq!(output[0], 0xf8); // Long list marker
        // assert!(output.len() > 55); // Verify it's actually a long list
    }
}