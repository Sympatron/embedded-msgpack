#![no_std]

pub mod decode;
pub mod encode;
pub mod ext;
mod marker;

pub use ext::*;

#[derive(Debug)]
pub enum Error {
    OutOfBounds,
    InvalidType,
    EndOfBuffer,
}
