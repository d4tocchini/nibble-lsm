#![feature(test)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(repr_simd)]

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Remove these XXX
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]

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
extern crate syscall;

// TODO keep cuckoo private and move the unit test in the integration
// test code to where it should be
pub mod cuckoo;

pub mod nibble;
pub use nibble::*;
