use inventory::submit;

pub mod solutions;

pub type InputIterator<'a, 'b> = &'a mut dyn Iterator<Item = &'b str>;
pub type Handler = fn(InputIterator) -> String;
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
