#![no_std]
#![feature(test)]
#[macro_use]
extern crate crypto_tests;
extern crate magma;

bench_block_cipher!(magma::Magma, 32);
