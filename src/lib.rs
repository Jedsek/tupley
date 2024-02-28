#![doc = include_str!("../README.md")]
#![allow(incomplete_features)]
#![cfg_attr(feature = "len-generic", feature(generic_const_exprs))]

pub mod macros;
pub mod tuple;

#[cfg(test)]
mod tests;

pub mod prelude {
    pub use crate::macros::*;
    pub use crate::tuple::*;
}
