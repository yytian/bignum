use types::*;
use types::Sign::*;
use basic_ops::*;
use std::cmp;

pub fn bignum_karatsuba_mult(a: &Bignum, b: &Bignum, cutoff: usize) -> Bignum {
    let sign = match (&a.sign, &b.sign) {
        (&Nonnegative, &Nonnegative) => Nonnegative,
        (&Nonnegative, &Negative) => Negative,
        (&Negative, &Nonnegative) => Negative,
        (&Negative, &Negative) => Nonnegative,
    };

    let result = karatsuba_rec(a, b, cutoff);

    Bignum {
        sign: sign,
        parts: result.parts,
    }
}

fn divide_round_up(a: usize, b: usize) -> usize {
    // http://stackoverflow.com/questions/17944/how-to-round-up-the-result-of-integer-division
    (a - 1) / b + 1
}

fn karatsuba_rec(a: &Bignum, b: &Bignum, cutoff: usize) -> Bignum {
    // c = a_h * b_h
    // d = a_l * b_l
    // e = (a_h + a_l)(b_h + b_l) - c - d
    // ab = c * r^n + e * r^n/2 + d

    assert!(cutoff >= 2);

    let p = a.parts.len();
    let q = b.parts.len();

    if p <= cutoff || q <= cutoff {
        return bignum_long_mult(a, b);
    }

    let m = divide_round_up(cmp::max(p, q), 2);

    let (a_l, a_h) = a.parts.split_at(cmp::min(m, p));
    let (b_l, b_h) = b.parts.split_at(cmp::min(m, q));

    let mut c;
    let d;
    let mut e;

    {
        let a_h_b = Bignum { sign: Nonnegative, parts: a_h.to_vec() };
        let a_l_b = Bignum { sign: Nonnegative, parts: a_l.to_vec() };
        let b_h_b = Bignum { sign: Nonnegative, parts: b_h.to_vec() };
        let b_l_b = Bignum { sign: Nonnegative, parts: b_l.to_vec() };
        c = karatsuba_rec(&a_h_b, &b_h_b, cutoff);
        d = karatsuba_rec(&a_l_b, &b_l_b, cutoff);
        e = bignum_sub(&bignum_sub(
            &karatsuba_rec(&bignum_add(&a_h_b, &a_l_b), &bignum_add(&b_h_b, &b_l_b), cutoff),
            &c), &d);

        // Falling out of this block drops the intermediate results
    }

    shift_left(&mut c, m * 2);
    shift_left(&mut e, m);
    bignum_add(&c, &bignum_add(&e, &d))
}
