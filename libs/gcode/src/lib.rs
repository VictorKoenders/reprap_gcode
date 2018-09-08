#![no_std]

#[macro_use]
mod macros;

mod data;
mod parser;

pub use data::*;
use parser::Parser;

pub type Result<T> = core::result::Result<T, Error>;

/// Parse an instruction from the given byte slice
pub fn parse(input: &[u8]) -> Result<Instruction> {
    let parser = match Parser::new(input) {
        Some(parser) => parser,
        None => return Err(Error::NotEnoughBytes),
    };

    unimplemented!()
}
