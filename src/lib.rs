use std::borrow::Cow;

use inventory::submit;

pub mod solutions;
pub mod parsers;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coord {
    row: i32,
    col: i32
}

pub type Ztr = Cow<'static, str>;
pub type InputIterator<'a> = &'a mut dyn Iterator<Item = Ztr>;
pub type Handler = fn(InputIterator, bool) -> Ztr;
pub struct Plugin(pub &'static str, pub Handler);

inventory::collect!(Plugin);

macro_rules! solution {
    ($name:tt) => {
        $crate::submit! {
            use crate::solutions::$name::solution;
            $crate::Plugin(stringify!($name), solution)
        }
    }
}

solution!(s00);
solution!(s01);
solution!(s02);
solution!(s03);
solution!(s04);
solution!(s05);
solution!(s06);
solution!(s07);
solution!(s08);
solution!(s09);
solution!(s10);
solution!(s12);
solution!(s14);
solution!(s18);
solution!(s17);
solution!(s23);
solution!(s24);
