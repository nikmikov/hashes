#![no_std]
#![feature(test)]

#[macro_use]
extern crate digest;
extern crate groestl;

bench_digest!(groestl::Groestl512);
