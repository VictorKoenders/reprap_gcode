use core::str;
use data::Error;
use Result;

pub struct Parser<'a> {
    data: &'a str,
    iter: str::Split<'a, &'static str>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &[u8]) -> Result<Parser> {
        let index = input
            .iter()
            .position(|i| *i == b' ')
            .ok_or(Error::NotEnoughBytes)?;
        let str = str::from_utf8(&input[..index])?;

        Ok(Parser {
            data: &str,
            iter: str.split(" "),
        })
    }

    pub fn get_float_pair(&mut self) -> Option<(u8, f32)> {}
    pub fn get_int_pair(&mut self) -> Option<(u8, u32)> {}
}
