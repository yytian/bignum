extern crate bignum;
#[macro_use]
extern crate quickcheck;

use bignum::types::*;
use bignum::basic_ops::*;
use bignum::karatsuba::*;
use std::cmp::Ordering;

/**
 * Tests which failed quickcheck at some point, to preserve the witnesses
 */
#[test]
fn regression_test_1() {
    let b1 = Bignum {
        sign: Sign::Nonnegative,
        parts: vec!(0, 0, 0, 0, 1),
    };
    let b2 = Bignum {
        sign: Sign::Negative,
        parts: vec!(0, 0, 0, 0, 0, 0, 0, 0, 1)
    };

    let long_mult = bignum_long_mult(&b1, &b2);
    let karatsuba_mult = bignum_karatsuba_mult(&b1, &b2, 4, true);
    assert_eq!(long_mult, karatsuba_mult);
}

fn bool_to_sign(b: bool) -> Sign {
    if b { Sign::Nonnegative } else { Sign::Negative }
}

#[test]
quickcheck! {
    fn long_mult_same_as_karatsuba(
        parts1: Vec<u32>,
        parts2: Vec<u32>,
        sign1: bool,
        sign2: bool
    ) -> bool {
        let b1 = Bignum {
            sign: bool_to_sign(sign1),
            parts: parts1,
        };

        let b2 = Bignum {
            sign: bool_to_sign(sign2),
            parts: parts2,
        };
        
        let long_mult = bignum_long_mult(&b1, &b2);
        let karatsuba_mult = bignum_karatsuba_mult(&b1, &b2, 4, true);
        long_mult.cmp(&karatsuba_mult) == Ordering::Equal
    }
}
