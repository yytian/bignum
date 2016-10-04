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

pub const BASE: u64 = 64;

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

fn char_to_digit(c: char) -> u64 {
    c as u64 - '0' as u64
}

fn digit_to_char(part: u64) -> char {
    ::std::char::from_u32((part + '0' as u64) as u32).unwrap()
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

        let mut quotient: String = s.to_string();
        let mut remainder: u64 = 0; // Should be < 64

        while quotient != "0" {
            // Repeated long division by 64
            let mut next = String::with_capacity(quotient.len());
            let mut carry = 0;
            
            for c in quotient.chars() {
                let digit = char_to_digit(c);
                carry = carry * 10 + digit;

                // TODO: Don't do the char conversion every time
                next.push(digit_to_char(carry / 64));
                carry = carry % 64;
            }
            quotient = skip_leading_zeroes(&next).to_string();
            parts.push(carry);
        }
        Ok(parts)
    }

    pub fn to_string(&self) -> String {
        let mut prefix: String = match self.sign {
            Negative => "-".to_string(),
            Nonnegative => "".to_string(),
        };
        let rest = self.parts.iter().rev();
        let mut total: u64 = 0;
        for part in rest {
            total = total * 64 + part;
        }

        let mut s = String::new();
        while total > 0 {
            s.push(digit_to_char(total % 10));
            total = total / 10
        }

        let num_str = s.chars().rev().collect::<String>();
        prefix.push_str(skip_leading_zeroes(&num_str));
        prefix
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
    let examples = vec!("0", "1", "-1", "63", "-69", "123", "-12345", "952892589210459282926222035");
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
