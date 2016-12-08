use std::cmp;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sign {
    Nonnegative = 1,
    Negative = -1,
}

use self::Sign::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bignum {
    pub parts: Vec<u32>, // Least significant digit at leftmost index
    pub sign: Sign,
}

#[derive(Debug)]
pub struct ParseBignumError;

// Max value of a single part
// Needs to be less than half the max value of the integer type
// To allow adding without overflow
pub const BASE: u32 = 2000000000;
pub const BASE_STR: &'static str = "2000000000";

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

fn char_to_digit(c: char) -> u32 {
    c as u32 - '0' as u32
}

fn digit_to_char(part: u32) -> char {
    ::std::char::from_u32(part + '0' as u32).unwrap()
}

fn string_add<'a, 'b>(left: &'a mut String, right: &'b str) -> &'a String {
    let l = left.chars().collect::<Vec<char>>();
    let r = right.chars().collect::<Vec<char>>();

    let p = l.len();
    let q = r.len();

    let max = cmp::max(p, q);

    let mut temp_str = String::with_capacity(max + 1);
    let mut carry: u32 = 0;
    for i in 1..max+1 {
        let l_digit = if p >= i {
            char_to_digit(l[p-i])
        } else {
            0
        };
        let r_digit = if q >= i {
            char_to_digit(r[q-i])
        } else {
            0
        };
        let result: u64 = (l_digit + r_digit + carry) as u64;
        temp_str.push(digit_to_char((result % 10) as u32));
        carry = (result / 10) as u32;
    }
    if carry > 0 {
        temp_str.push(digit_to_char(carry));
    }

    left.clone_from(&temp_str.chars().rev().collect::<String>());
    left
}

fn string_mult<'a, 'b>(left: &'a mut String, right: &'b str) -> &'a String {
    let l = left.chars().collect::<Vec<char>>();
    let r = right.chars().collect::<Vec<char>>();

    let p = l.len();
    let q = r.len();

    let (top, bot) = if p >= q {
        (l, r)
    } else {
        (r, l)
    };

    let min = cmp::min(p, q);
    let max = cmp::max(p, q);

    let mut temp_str = String::with_capacity(p + q);
    temp_str.push('0');
    let mut carry: u32 = 0;
    for i in 1..min+1 {
        // For each digit of the bottom, multiply by the top
        let mut line_str = String::with_capacity(max + 1);
        for j in 1..max+1 {
            let top_digit = if max >= j {
                char_to_digit(top[max-j])
            } else {
                0
            };
            let bot_digit = if min >= i {
                char_to_digit(bot[min-i])
            } else {
                0
            };
            let result: u64 = (top_digit * bot_digit + carry) as u64;
            line_str.push(digit_to_char((result % 10) as u32));
            carry = (result / 10) as u32;
        }

        while carry > 0 {
            line_str.push(digit_to_char(carry));
            carry = carry / 10;
        }
        
        line_str = line_str.chars().rev().collect::<String>();
        // Move result forward
        for _ in 0..i-1 {
            line_str.push('0');
        }

        string_add(&mut temp_str, &line_str);
    }

    left.clone_from(&skip_leading_zeroes(&temp_str).to_string());
    left
}

pub fn from_string(input_str: &str) -> Result<Bignum, ParseBignumError> {
    if input_str.is_empty() {
        Err(ParseBignumError)
    }
    else if input_str.starts_with("-") {
        string_to_parts(&input_str[1..]).map(|parts| Bignum {
            parts: parts,
            sign: Negative,
        })
    }
    else {
        string_to_parts(&input_str[..]).map(|parts| Bignum {
            parts: parts,
            sign: Nonnegative,
        })
    }
}

fn string_to_parts(input_string: &str) -> Result<Vec<u32>, ParseBignumError> {
    let s = skip_leading_zeroes(input_string);
    let mut parts = Vec::with_capacity(s.len());

    let mut quotient: String = s.to_string();

    while quotient != "0" {
        // Repeated long division by BASE
        let mut next = String::with_capacity(quotient.len());
        let mut carry: u64 = 0;
        
        for c in quotient.chars() {
            let digit = char_to_digit(c);
            carry = carry * 10 + digit as u64;

            // TODO: Don't do the char conversion every time
            next.push(digit_to_char((carry / BASE as u64) as u32));
            carry = carry % BASE as u64;
        }
        quotient = skip_leading_zeroes(&next).to_string();
        parts.push(carry as u32);
    }
    Ok(parts)
}

pub fn shift_left(a: &mut Bignum, num_places: usize) {
    // TODO: Possibly impl as a << trait?
    let mut zeroes = vec![0; num_places];
    zeroes.append(&mut a.parts);
    a.parts = zeroes;
}

impl Bignum {
    pub fn normalize(&mut self) {
        let mut last_non_zero_index = 0;
        let num_parts = self.parts.len();
        for i in 0..num_parts {
            let index_to_test = num_parts - i - 1;
            if self.parts[index_to_test] != 0 {
                last_non_zero_index = index_to_test;
                break;
            }
        }
        if last_non_zero_index != num_parts - 1 {
            self.parts.truncate(last_non_zero_index + 1);
        }
    }
    
    pub fn to_string(&self) -> String {
        let mut prefix: String = match self.sign {
            Negative => "-".to_string(),
            Nonnegative => "".to_string(),
        };

        let rest = self.parts.iter().rev();

        // Repeatedly multiply by 64
        let mut product: String = String::from("0");

        for part in rest {
            let mut next: String = String::with_capacity(2);
            next.push_str(&part.to_string());

            string_mult(&mut product, BASE_STR);
            string_add(&mut product, &next);
        }

        prefix.push_str(&product);
        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::string_add;
    use super::string_mult;
    use super::from_string;
    use super::shift_left;
    use super::BASE_STR;
    
    #[test]
    fn string_add_test() {
        assert_eq!(string_add(&mut "123".to_string(), "123"), "246");
        assert_eq!(string_add(&mut "123".to_string(), "0"), "123");
        assert_eq!(string_add(&mut "123".to_string(), "10000"), "10123");
        assert_eq!(string_add(&mut "123456789".to_string(), "987654321"), "1111111110");
    }

    #[test]
    fn string_mult_test() {
        assert_eq!(string_mult(&mut "3".to_string(), "3"), "9");
        assert_eq!(string_mult(&mut "0".to_string(), "999"), "0");
        assert_eq!(string_mult(&mut "123".to_string(), "241"), "29643");
        assert_eq!(string_mult(&mut "349".to_string(), "807"), "281643");
        assert_eq!(string_mult(&mut "55555".to_string(), "66666"), "3703629630");
    }

    #[test]
    fn type_conversion_test() {
        let examples = vec!(
            "0",
            "1",
            "-1",
            "63",
            "-69",
            "-9877",
            "11111111111111111111",
            "-952892589210459282926222035",
            "12193263132251181129",
        );
        for string_rep in examples {
            let big = from_string(string_rep).unwrap();
            assert_eq!(string_rep, big.to_string());
        }
    }

    #[test]
    fn equality_test() {
        assert!(from_string("123").unwrap() == from_string("123").unwrap());
        assert!(from_string("123").unwrap() != from_string("-123").unwrap());
        assert!(from_string("123").unwrap() != from_string("124").unwrap());
        // TODO: check for leading zeroes
    }

    #[test]
    fn shift_left_test() {
        let num: u64 = 123;
        let mut b = from_string(&num.to_string()).unwrap();
        shift_left(&mut b, 3); // multiply by base 3 times

        // Less ugly way to do this?
        let mut str = num.to_string();
        let mut once = string_mult(&mut str, BASE_STR).to_string();
        let mut twice = string_mult(&mut once, BASE_STR).to_string();
        let thrice = string_mult(&mut twice, BASE_STR).to_string();
        assert_eq!(b.to_string(), thrice.to_owned());
    }
}
