use crate::prelude::Find;

#[derive(PartialEq, Debug)]
pub enum NumberSystem {
    Binary, Octal, Decimal, Hexadecimal
}
impl From<&[u8]> for NumberSystem {
    fn from(value: &[u8]) -> Self {
        if value.len() < 2 {
            NumberSystem::Decimal
        } else {
            if value[0] == b'0' {
                if value[1] == b'x' || value[1] == b'X' {
                    NumberSystem::Hexadecimal
                } else if value[1] == b'o' || value[1] == b'O' {
                    NumberSystem::Octal
                } else if value[1] == b'b' || value[1] == b'B' {
                    NumberSystem::Binary
                } else {
                    NumberSystem::Decimal
                }
            } else  { NumberSystem::Decimal }
        }
    }
}

impl From<u8> for NumberSystem {
    fn from(value: u8) -> Self {
        if value == b'x' || value == b'X' {
            NumberSystem::Hexadecimal
        } else if value == b'o' || value == b'O' {
            NumberSystem::Octal
        } else if value == b'b' || value == b'B' {
            NumberSystem::Binary
        } else { NumberSystem::Decimal }
    }
}

pub trait Trim {
    fn trim(&mut self);
    fn del_zero(&mut self);
    fn trim_zero(&mut self);
}

fn is_white_space(ch: char) -> bool {
    let white_space = [' ', '\n', '\r', '\t'];
    for c in white_space {
        if c == ch {
            return true
        }
    }
    false
}

impl Trim for Vec<u8> {
    fn trim(&mut self) {
        if self.is_empty() { return; }
        while !self.is_empty() {
            let len = self.len() - 1;
            let b = self[len];
            if is_white_space(b as char) {
                self.pop();
            } else { break; }
        }
        while !self.is_empty() {
            if is_white_space(self[0] as char) {
                self.remove(0);
            } else { break }
        }
    }



    fn del_zero(&mut self) {
        if self.len() < 2 { return; }
        let mut start = 0;
        let mut len;
        match NumberSystem::from(self.as_slice()) {
            NumberSystem::Decimal => {
                if self[0] == b'-' { start += 1; }
                match self.find(&b'.') {
                    Some(k) => {
                        len = k;
                        while k + 2 < self.len() {
                            if self[self.len() - 1] == b'0' {
                                self.pop();
                            } else { break }
                        }
                    }
                    _ => len = self.len()
                }
            }
            _ => {
                start = 2;
                len = self.len();
            }
        }
        while len > start + 1 {
            if self[start] == b'0' {
                self.remove(start);
                len -= 1;
            } else { break }
        }
        if self.ends_with(&vec![b'.']) { self.push(b'0') }
    }

    fn trim_zero(&mut self) {
        self.trim();
        self.del_zero();
    }
}



