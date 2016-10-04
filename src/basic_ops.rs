use types::*;
use types::Sign::*;
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

impl Bignum {
    // pub fn add(a: &Bignum, b: &Bignum) -> Bignum {
    //     Bignum::from_string("0").unwrap()
    // }
    
    // pub fn sub(a: &Bignum, b: &Bignum) -> Bignum {
    //     Bignum::from_string("0").unwrap()
    // }
    
    pub fn long_mult(a: &Bignum, b: &Bignum) -> Bignum {
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
}

fn try_with_strs(f: fn(&Bignum, &Bignum) -> Bignum, a1: &str, a2: &str) -> String {
    let b1 = Bignum::from_string(a1).unwrap();
    let b2 = Bignum::from_string(a2).unwrap();
    f(&b1, &b2).to_string()
}

#[test]
fn comparison_test() {
    let b1 = Bignum::from_string("123").unwrap();
    let b2 = Bignum::from_string("234").unwrap();
    let b3 = Bignum::from_string("-234").unwrap();
    let b4 = Bignum::from_string("0").unwrap();
    let b5 = Bignum::from_string("-000").unwrap();
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
fn long_mult_test() {
    assert_eq!(try_with_strs(Bignum::long_mult, "2", "2"), "4");
    assert_eq!(try_with_strs(Bignum::long_mult, "-2", "2"), "-4");
    assert_eq!(try_with_strs(Bignum::long_mult, "-2", "-2"), "4");
    assert_eq!(try_with_strs(Bignum::long_mult, "123456789", "987654321"), "121932631112635269");
}
