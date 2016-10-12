use types::*;
use types::Sign::*;
use std::cmp;
use std::cmp::Ordering;

// TODO: Trim leading zeroes in intermediate forms?

impl Bignum {
    fn is_zero(&self) -> bool {
        for p in &self.parts {
            if *p != 0 {
                return false;
            }
        }
        return true;
    }
    
    fn cmp_sign(&self, other: &Bignum) -> Ordering {
        self.sign.cmp(&other.sign)
    }

    fn cmp_parts(&self, other: &Bignum) -> Ordering {
        let (left, right) = match self.sign {
            Nonnegative => (Ordering::Greater, Ordering::Less),
            Negative => (Ordering::Less, Ordering::Greater),
        };
    

        let p = self.parts.len();
        let q = other.parts.len();

        // Assume no leading zeroes
        if p > q {
            return left;
        } else if p < q {
            return right;
        }

        // Same number of digits
        let self_digits = self.parts.iter();
        let other_digits = other.parts.iter();
        let mut zipped = self_digits.zip(other_digits).rev();
    
        loop {
            match zipped.next() {
                Some((self_digit, other_digit)) =>
                    if self_digit > other_digit {
                        return left;
                    } else if self_digit < other_digit {
                        return right;
                    },
                None => return Ordering::Equal,
            }
        }
    }
}

impl PartialOrd for Bignum {
    fn partial_cmp(&self, other: &Bignum) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Bignum {
    fn cmp(&self, other: &Bignum) -> Ordering {
        if self.is_zero() && other.is_zero() {
            return Ordering::Equal;
        }

        let sign_order = self.cmp_sign(other);
        if sign_order == Ordering::Equal {
            self.cmp_parts(other)
        } else {
            sign_order
        }
    }
}

pub fn bignum_add(a: &Bignum, b: &Bignum) -> Bignum {
    let parts_ord = a.cmp_parts(b);
    let sign = match (&a.sign, &b.sign, parts_ord) {
        (&Nonnegative, &Nonnegative, _) => Nonnegative,
        (&Nonnegative, &Negative, Ordering::Less) => Negative,
        (&Nonnegative, &Negative, _) => Nonnegative,
        (&Negative, &Nonnegative, Ordering::Greater) => Negative,
        (&Negative, &Nonnegative, _) => Nonnegative,
        (&Negative, &Negative, _) => Nonnegative,
    };

    let should_sub = a.sign != b.sign;
    
    let p = a.parts.len();
    let q = b.parts.len();

    let max = cmp::max(p, q);
    
    let mut sum = Bignum {
        sign: sign,
        parts: Vec::with_capacity(max + 1),
    };

    // TODO: make this prettier
    if !should_sub {
        let mut carry = 0;
        for i in 0..max {
            let a_digit = if i < p {
                a.parts[i]
            } else {
                0
            };
            let b_digit = if i < q {
                b.parts[i]
            } else {
                0
            };
            let result = a_digit + b_digit + carry;
            sum.parts.push(result % BASE);
            carry = result / BASE;
        }
        if carry > 0 {
            sum.parts.push(carry);
        }
    } else {
        let mut carry = 0;
        let (big, small) = match a.cmp_parts(b) {
            Ordering::Less => (b, a),
            _ => (a, b),
        };
        
        for i in 0..max {
            let big_digit = if i < big.parts.len() {
                big.parts[i]
            } else {
                0
            };
            let small_digit = if i < small.parts.len() {
                small.parts[i]
            } else {
                0
            };

            let result = if (big_digit - carry) < small_digit {
                let temp = big_digit - carry + BASE - small_digit;
                carry = 1;
                temp
            } else {
                let temp = big_digit - carry - small_digit;
                carry = 0;
                temp
            };
            
            sum.parts.push(result);
        }
        assert!(carry == 0);
    }
    
    sum
}

pub fn bignum_sub(a: &Bignum, b: &Bignum) -> Bignum {
    let neg_b = Bignum {
        sign: match b.sign {
            Negative => Nonnegative,
            Nonnegative => Negative
        },
        parts: b.parts.clone(),
    };

    bignum_add(a, &neg_b)
}

pub fn bignum_long_mult(a: &Bignum, b: &Bignum) -> Bignum {
    // https://en.wikipedia.org/wiki/Multiplication_algorithm#Long_multiplication
    let p = a.parts.len();
    let q = b.parts.len();
    let mut product = Bignum {
        sign: match (&a.sign, &b.sign) {
            (&Nonnegative, &Nonnegative) => Nonnegative,
            (&Nonnegative, &Negative) => Negative,
            (&Negative, &Nonnegative) => Negative,
            (&Negative, &Negative) => Nonnegative,
        },
        parts: vec!(0; p + q),
    };
    
    for b_i in 0..q {
        let mut carry = 0;
        for a_i in 0..p {
            product.parts[a_i + b_i] += carry + a.parts[a_i] * b.parts[b_i];
            carry = product.parts[a_i + b_i] / BASE;
            product.parts[a_i + b_i] = product.parts[a_i + b_i] % BASE;
        }
        product.parts[b_i + p] += carry;
    }
    product
}

#[cfg(test)]
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
