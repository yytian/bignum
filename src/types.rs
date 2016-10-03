#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sign {
    Nonnegative = 1,
    Negative = -1,
}

use self::Sign::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Bignum {
    pub parts: Vec<u64>, // Least significant digit at leftmost index
    pub sign: Sign,
}

#[derive(Debug)]
pub struct ParseBignumError;

pub const BASE: u64 = 10;

fn skip_leading_zeroes(s: &str) -> &str {
    let mut chars = s.chars();
    let mut first_nonzero_index = 0;

    loop {
        let c = chars.next();
        if let Some('0') = c {
            first_nonzero_index += 1;
        } else {
            break;
        }
    }

    if first_nonzero_index >= s.len() {
        "0"
    } else {
        &s[first_nonzero_index..]
    }
}

impl Bignum {
    pub fn from_string(input_str: &str) -> Result<Self, ParseBignumError> {
        if input_str.is_empty() {
            Err(ParseBignumError)
        }
        else if input_str.starts_with("-") {
            Bignum::string_to_parts(&input_str[1..]).map(|parts| Bignum {
                parts: parts,
                sign: Negative,
            })
        }
        else {
            Bignum::string_to_parts(&input_str[..]).map(|parts| Bignum {
                parts: parts,
                sign: Nonnegative,
            })
        }
    }

    fn string_to_parts(input_string: &str) -> Result<Vec<u64>, ParseBignumError> {
        let s = skip_leading_zeroes(input_string);
        let mut parts = Vec::with_capacity(s.len());
        for c in s.chars().rev() {
            parts.push(c as u64 - '0' as u64);
        }
        Ok(parts)
    }

    fn to_utf8(part: &u64) -> char {
        ::std::char::from_u32((*part + '0' as u64) as u32).unwrap()
    }

    pub fn to_string(&self) -> String {
        let prefix = match self.sign {
            Negative => "-".to_string(),
            Nonnegative => "".to_string(),
        };
        let rest = self.parts.iter().rev().map(Bignum::to_utf8).collect::<String>();
        // TODO: Trim leading zeroes in intermediate forms?
        skip_leading_zeroes(&(prefix + &rest)).to_owned()
    }
/*
    pub fn karatsuba_mult(a: &Bignum, b: &Bignum) -> Bignum {
        // https://en.wikipedia.org/wiki/Karatsuba_algorithm#Basic_step
        // xy = (b^2 + b)x_1y_1 - b(x_1 - x_0)(y_1 - y_0) + (b + 1)x_0y_0
        // where b = B^m
        
    }
*/
}

#[test]
fn type_conversion_test() {
    let examples = vec!("0", "1", "-1", "-12345", "952892589210459282926222035");
    for string_rep in examples {
        let big = Bignum::from_string(string_rep).unwrap();
        assert_eq!(string_rep, big.to_string());
    }
}

#[test]
fn equality_test() {
    assert!(Bignum::from_string("123").unwrap() == Bignum::from_string("123").unwrap());
    assert!(Bignum::from_string("123").unwrap() != Bignum::from_string("-123").unwrap());
    assert!(Bignum::from_string("123").unwrap() != Bignum::from_string("124").unwrap());
    // TODO: check for leading zeroes
}
