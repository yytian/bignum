#![feature(test)]

extern crate bignum;
extern crate test;

use bignum::types::*;
use bignum::basic_ops::*;
use bignum::karatsuba::*;
use test::Bencher;

fn try_long_mult(bencher: &mut Bencher, num_nines: usize) {
    let nines = std::iter::repeat("9").take(num_nines).collect::<String>();
    let a = from_string(&nines).unwrap();
    let b = a.clone();
    bencher.iter(|| bignum_long_mult(&a, &b));
}

#[bench]
fn long_mult_thousand(bencher: &mut Bencher) {
    try_long_mult(bencher, 1000);
}

#[bench]
fn long_mult_five_thousand(bencher: &mut Bencher) {
    try_long_mult(bencher, 5000);
}

fn try_karatsuba_mult(bencher: &mut Bencher, cutoff: usize, num_nines: usize) {
    let nines = std::iter::repeat("9").take(num_nines).collect::<String>();
    let a = from_string(&nines).unwrap();
    let b = a.clone();
    bencher.iter(|| bignum_karatsuba_mult(&a, &b, cutoff));
}

#[bench]
fn karatsuba_thousand_cutoff_twenty(bencher: &mut Bencher) {
    try_karatsuba_mult(bencher, 20, 1000);
}

#[bench]
fn karatsuba_thousand_cutoff_fifty(bencher: &mut Bencher) {
    try_karatsuba_mult(bencher, 50, 1000);
}

#[bench]
fn karatsuba_thousand_cutoff_hundred(bencher: &mut Bencher) {
    try_karatsuba_mult(bencher, 100, 1000);
}

#[bench]
fn karatsuba_five_thousand_cutoff_twenty(bencher: &mut Bencher) {
    try_karatsuba_mult(bencher, 20, 5000);
}

#[bench]
fn karatsuba_five_thousand_cutoff_fifty(bencher: &mut Bencher) {
    try_karatsuba_mult(bencher, 50, 5000);
}

#[bench]
fn karatsuba_five_thousand_cutoff_hundred(bencher: &mut Bencher) {
    try_karatsuba_mult(bencher, 100, 5000);
}

