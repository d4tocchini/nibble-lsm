#![feature(test)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(repr_simd)]

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

extern crate libc;
extern crate rand;
extern crate test;
extern crate time;
extern crate crossbeam;
extern crate itertools;
extern crate quicksort;

pub mod nibble;
pub use nibble::*;
