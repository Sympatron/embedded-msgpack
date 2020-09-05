#![no_std]

pub mod decode;
pub mod encode;
#[cfg(feature = "ext")]
pub mod ext;
mod marker;

#[cfg(feature = "ext")]
pub use ext::*;
pub use serde_bytes::Bytes;
