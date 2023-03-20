use std::str::{from_utf8, FromStr};

pub use pom::parser::{self, one_of, Parser};
use pom::parser::{end, list};

pub fn space<'a>() -> Parser<'a, u8, ()> {
    one_of(b" \t").repeat(0..).discard()
}

pub fn newline<'a>() -> Parser<'a, u8, ()> {
    one_of(b"\r\n").repeat(0..).discard()
}

pub fn integer<'a>() -> Parser<'a, u8, i32> {
    let integer = one_of(b"0123456789") - one_of(b"0123456789").repeat(0..);
    integer.collect().convert(from_utf8).convert(|s| i32::from_str(&s))
}

fn itu() -> Parser<'static, u8, (i32, i32)> {
    let parser = (integer() - space()).repeat(2) - end();
    parser.map(|v| (v[0], v[1]))
}

#[test]
fn tuple() {
    let output = itu().parse(b"3 4");
    assert_eq!(output, Ok((3, 4)));
}

#[test]
fn two_numbers() {
    let parser = list(integer(), space()) - end();
    let output = parser.parse(b"11 22");
    assert_eq!(output, Ok(vec![11, 22]));
}
