#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;

mod hierarchy;
pub use hierarchy::*;

mod maybe;
pub use maybe::*;

mod template;
pub use template::*;
