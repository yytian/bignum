enum Sign {
    Nonnegative,
    Negative,
}

use self::Sign::*;

pub struct Bignum {
    parts: Vec<u64>, // Least significant digit at leftmost index
    sign: Sign,
}

#[derive(Debug)]
pub struct ParseBignumError;

const BASE: u64 = 10;

impl Bignum {
    pub fn from_string(input_str: &str) -> Result<Self, ParseBignumError> {
        if input_str.is_empty() {
            Err(ParseBignumError)
        }
        else if input_str.starts_with("-") {
            Ok(Bignum {
                parts: Bignum::string_to_parts(&input_str[1..]),
                sign: Negative,
            })
        }
        else {
            Ok(Bignum {
                parts: Bignum::string_to_parts(&input_str[..]),
                sign: Nonnegative,
            })
        }
    }

    fn string_to_parts(input_string: &str) -> Vec<u64> {
        let mut parts = Vec::with_capacity(input_string.len());
        for c in input_string.chars().rev() {
            parts.push(c.to_digit(BASE as u32).unwrap() as u64);
        }
        parts
    }

    fn to_utf8(part: &u64) -> char {
        ::std::char::from_u32(*part as u32 + '0' as u32).unwrap()
    }

    pub fn to_string(&self) -> String {
        let prefix = match self.sign {
            Negative => "-".to_string(),
            Nonnegative => "".to_string(),
        };
        let rest = self.parts.iter().rev().map(Bignum::to_utf8).collect::<String>();
        prefix + &rest
    }

    pub fn long_mult(&self, other: &Bignum) -> Bignum {
        // https://en.wikipedia.org/wiki/Multiplication_algorithm#Long_multiplication
        let p = self.parts.len();
        let q = other.parts.len();
        let mut product = Bignum {
            sign: match (&self.sign, &other.sign) {
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
                product.parts[a_i + b_i] += carry + self.parts[a_i] * other.parts[b_i];
                carry = product.parts[a_i + b_i] / BASE;
                product.parts[a_i + b_i] = product.parts[a_i + b_i] % BASE;
            }
            product.parts[b_i + p] += carry;
        }
        product
    }
}

#[test]
fn type_conversions() {
    let examples = vec!("0", "1", "-1", "-12345", "952892589210459282926222035");
    for string_rep in examples {
        let big = Bignum::from_string(string_rep).unwrap();
        assert_eq!(string_rep, big.to_string());
    }
}

#[test]
fn long_mult_test() {
    let num1 = Bignum::from_string("123456789").unwrap();
    let num2 = Bignum::from_string("987654321").unwrap();
    let product = num1.long_mult(&num2);
    let string_rep = product.to_string();
    assert_eq!(string_rep, "121932631112635269");
}
