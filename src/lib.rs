use std::borrow::Cow;
use std::io::BufRead;
use inventory::submit;

pub mod solutions;
pub mod parsers;

pub type Ztr = Cow<'static, str>;
pub type InputIterator<'a> = &'a mut dyn Iterator<Item = Ztr>;
pub type Handler = fn(InputIterator) -> (Ztr, Ztr);
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
