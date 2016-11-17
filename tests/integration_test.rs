extern crate bignum;

use bignum::types::*;
use bignum::basic_ops::*;
use std::cmp::Ordering;

fn try_with_strs(f: fn(&Bignum, &Bignum) -> Bignum, a1: &str, a2: &str) -> String {
    let b1 = from_string(a1).unwrap();
    let b2 = from_string(a2).unwrap();
    f(&b1, &b2).to_string()
}

#[test]
fn comparison_test() {
    let b1 = from_string("123").unwrap();
    let b2 = from_string("234").unwrap();
    let b3 = from_string("-234").unwrap();
    let b4 = from_string("0").unwrap();
    let b5 = from_string("-000").unwrap();
    assert_eq!(b1.cmp(&b2), Ordering::Less);
    assert_eq!(b2.cmp(&b1), Ordering::Greater);
    assert_eq!(b3.cmp(&b4), Ordering::Less);
    assert_eq!(b1.cmp(&b1), Ordering::Equal);
    assert_eq!(b4.cmp(&b4), Ordering::Equal);
    assert_eq!(b4.cmp(&b5), Ordering::Equal);
    assert_eq!(b5.cmp(&b1), Ordering::Less);
    assert_eq!(b5.cmp(&b3), Ordering::Greater);
}

#[test]
fn bignum_add_test() {
    assert_eq!(try_with_strs(bignum_add, "123", "123"), "246");
    assert_eq!(try_with_strs(bignum_add, "123", "0"), "123");
    assert_eq!(try_with_strs(bignum_add, "123", "10000"), "10123");
    assert_eq!(try_with_strs(bignum_add, "123456789", "987654321"), "1111111110");
}

#[test]
fn bignum_sub_test() {
    assert_eq!(try_with_strs(bignum_sub, "123", "123"), "0");
    assert_eq!(try_with_strs(bignum_sub, "123", "0"), "123");
    assert_eq!(try_with_strs(bignum_sub, "123", "10000"), "-9877");
    assert_eq!(try_with_strs(bignum_sub, "123456789", "987654321"), "-864197532");
}

#[test]
fn long_mult_test() {
    assert_eq!(try_with_strs(bignum_long_mult, "2", "2"), "4");
    assert_eq!(try_with_strs(bignum_long_mult, "-2", "2"), "-4");
    assert_eq!(try_with_strs(bignum_long_mult, "-2", "-2"), "4");
    assert_eq!(try_with_strs(bignum_long_mult, "123456789", "987654321"), "121932631112635269");
}
