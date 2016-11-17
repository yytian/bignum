use types::*;
use types::Sign::*;

pub fn bignum_karatsuba_mult(a: &Bignum, b: &Bignum) {
    // c = a_h * b_h
    // d = a_l * b_l
    // e = (a_h + a_l)(b_h + b_l) - c - d
    // ab = c * r^n + e * r^n/2 + d

}

#[test]
fn bignum_karatsuba_mult_test() {

}
