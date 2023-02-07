#![allow(unused)]

use std::mem::{size_of, size_of_val};
use std::thread::yield_now;

mod demand;
mod market;
mod materials;
mod pops;

fn main() {
    dbg!(size_of::<pops::Person>());
}
