//! RLP (Recursive Length Prefix) encoding and decoding.
//!
//! This crate implements RLP, a serialization format used extensively in Ethereum's protocol.
//! For more details on the RLP spec, see: https://ethereum.org/en/developers/docs/data-structures-and-encoding/rlp
//! 
//! # Examples
//! ```
//! use rrlp::{encode};
//! 
// ! let data = vec![1u8, 2u8, 3u8];
// ! let encoded = encode(&data);
// ! let decoded: Vec<u8> = decode(&encoded).unwrap();
// ! assert_eq!(data, decoded);
//! ```

#![deny(missing_docs, unsafe_code)]

// mod decode;
mod encode;
mod error;

// pub use decode::decode;
pub use encode::encode;
pub use error::Error;

// The maximum length of an RLP encode-able value, in bytes.
// This helps prevent OOM for malformed inputs.
const MAX_LENGTH: usize = 0x0fff_ffff;