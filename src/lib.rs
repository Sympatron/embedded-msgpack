#![cfg_attr(not(feature = "std"), no_std)]

extern crate zerocopy;

pub mod decode;
pub mod encode;
#[cfg(feature = "ext")]
pub mod ext;
mod marker;

#[cfg(feature = "ext")]
pub use ext::*;
pub use serde_bytes::Bytes;
