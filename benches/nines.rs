#![feature(test)]

extern crate bignum;
extern crate test;

use bignum::types::*;
use bignum::basic_ops::*;
use bignum::karatsuba::*;
use test::Bencher;

#[bench]
fn bench_long_mult(bencher: &mut Bencher) {
    let nines = std::iter::repeat("9").take(1000).collect::<String>();
    let a = from_string(&nines).unwrap();
    let b = a.clone();
    bencher.iter(|| bignum_long_mult(&a, &b));
}

#[bench]
fn bench_karatsuba_mult(bencher: &mut Bencher) {
    let nines = std::iter::repeat("9").take(1000).collect::<String>();
    let a = from_string(&nines).unwrap();
    let b = a.clone();
    bencher.iter(|| bignum_karatsuba_mult(&a, &b, 80));
}
