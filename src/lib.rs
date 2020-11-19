#![cfg_attr(not(test), no_std)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::cast_lossless)]

#[cfg(test)]
#[macro_use]
extern crate core;

#[cfg(test)]
#[macro_use]
extern crate libc_print;

pub mod sip;
pub mod sip128;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod tests128;
