#![feature(test)]

extern crate bignum;
extern crate test;

use bignum::types::*;
use bignum::basic_ops::*;

#[cfg(test)]
mod tests {
    use test::Bencher;

    #[bench]
    fn bench_long_mult(b: &mut Bencher) {
        let a = from_string("123");
        let b = from_string("234");
        b.iter(|| long_mult(a, b));
    }
}
