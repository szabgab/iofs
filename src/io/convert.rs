use std::io::{Error, ErrorKind};

use super::number::NumberSystem;
pub type ConResult<T> = Result<T, ConvertError>;

#[derive(Debug)]
pub enum ConvertError {
    Empty,
    Invalid,
    IoError(Error),
}
impl From<Error> for ConvertError {
    fn from(value: Error) -> Self {
        ConvertError::IoError(value)
    }
}

impl Into<Error> for ConvertError {
    fn into(self) -> Error {
        match self {
            ConvertError::IoError(e) => e,
            ConvertError::Invalid => Error::new(ErrorKind::Other, "ConvertError::Invalid"),
            ConvertError::Empty => Error::new(ErrorKind::Other, "ConvertError::Empty"),
        }
    }
}



pub trait ConvertBuffer: Sized {
    fn from_buf(buf: Vec<u8>) -> ConResult<Self>;
}


impl ConvertBuffer for String {
    fn from_buf(buf: Vec<u8>) -> ConResult<Self> {
        match String::from_utf8(buf) {
            Ok(value) => Ok(value),
            _ => Err(ConvertError::Invalid),
        }
    }
}

impl ConvertBuffer for Vec<u8> {
    fn from_buf(buf: Vec<u8>) -> ConResult<Self> {
        Ok(buf)
    }
}



macro_rules! derive_f {
    (for $($t:ty), +) => {
        $(impl ConvertBuffer for $t {
            fn from_buf(buf: Vec<u8>) -> ConResult<Self> {
                let buf = String::from_buf(buf)?;
                    match buf.trim().parse() {
                        Ok(value) => Ok(value),
                        _ => Err(ConvertError::Invalid)
                    }
            }
        })*
    };
}
derive_f!(for f32, f64);

macro_rules! derive_num {
    (for $($t:ty), +) => {
        $(impl ConvertBuffer for $t {
            fn from_buf(buf: Vec<u8>) -> ConResult<Self> {
                let buf = String::from_buf(buf)?;
                let bytes = buf.trim().as_bytes();
                let invalid = ConvertError::Invalid;
                if bytes.is_empty() { return Err(ConvertError::Empty); }
                if bytes[0] == b'-' {
                    match buf.parse() {
                        Ok(value) => Ok(value),
                        _ => Err(invalid)
                    }
                } else {
                    let mut number = 0;
                    match NumberSystem::from(bytes) {
                        NumberSystem::Decimal => {
                            return match buf.parse() {
                                Ok(value) => Ok(value),
                                _ => Err(ConvertError::Invalid)
                            }
                        }
                        NumberSystem::Hexadecimal => {
                            for p in 2..bytes.len() {
                                let Some(n) = hex_to_dec(bytes[p]) else { return Err(invalid) };
                                number = number * 16 + n as Self;
                            }
                        }
                        NumberSystem::Octal => {
                            for p in 2..bytes.len() {
                                if bytes[p] >= b'0' && bytes[p] <= b'7' {
                                    number = number * 8 + (bytes[p] - 48) as Self;
                                } else {
                                    return Err(invalid)
                                }
                            }
                        }
                        NumberSystem::Binary => {
                            for p in 2..bytes.len() {
                                if bytes[p] == b'0' || bytes[p] == b'1' {
                                    number = number * 2 + (bytes[p] - 48) as Self;
                                } else {
                                    return Err(invalid)
                                }
                            }
                        }
                    }
                    Ok(number)
                }
            }
        })*
    };
}
derive_num!(
    for i8, i16, i32, isize, i64, i128, u8, u16, u32, usize, u64, u128
);




#[allow(unused)]
fn is_too_big(max: &str, num: &str) -> bool {
    let num = del_front_zero(num);
    if num.len() == max.len() {
        let max = max.as_bytes();
        let num = num.as_bytes();
        for i in 0..num.len() {
            if max[i] < num[i] {
                return false;
            }
        }
        true
    } else {
        max.len() < num.len()
    }
}
#[allow(unused)]
fn is_too_small(min: &str, num: &str) -> bool {
    is_too_big(&min[1..], &num[1..])
}

// fn f_is_too_big(max: &str, num: &str) -> bool {
//     let dot = num.as_bytes().find(&b'.').unwrap_or(num.len());
//     let n = del_front_zero(&num[..dot]);
//     if max == n {
//         dot == num.len() || dot == num.len() - 1 || is_all_zero(&num[dot + 1..])
//     } else if max.len() == num.len() {
//         is_too_big(max, n)
//     } else {
//         max.len() > n.len()
//     }
// }
// fn f_is_too_small(min: &str, num: &str) -> bool {
//     f_is_too_big(&min[1..], &num[1..])
// }
// fn is_all_zero(dec: &str) -> bool {
//     for item in dec.as_bytes() {
//         if *item != b'0' {
//             return false;
//         }
//     }
//     true
// }

fn del_front_zero(number: &str) -> &str {
    let mut index = 0;
    for item in number.as_bytes() {
        if *item != b'0' {
            break;
        }
        index += 1;
    }
    if index == number.len() {
        "0"
    } else {
        &number[index..]
    }
}


fn hex_to_dec(byte: u8) -> Option<u8> {
    if byte >= b'0' && byte <= b'9' {
        Some(byte - b'0')
    } else if byte >= b'a' && byte <= b'f' {
        Some(byte - b'a' + 10)
    } else if byte >= b'A' && byte <= b'F' {
        Some(byte - b'A' + 10)
    } else {
        None
    }
}
