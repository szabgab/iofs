use crate::prelude::range::Range;

pub trait Symbols {
    fn is_letter(&self) -> bool {
        self.is_lower() || self.is_upper()
    }
    fn is_number(&self) -> bool;
    fn is_upper(&self) -> bool;
    fn is_lower(&self) -> bool;
    fn to_upper(&self) -> char;
    fn to_lower(&self) -> char;
}

impl Symbols for char {

    fn is_number(&self) -> bool {
        self.range_at(&'0', &'1')
    }

    fn is_upper(&self) -> bool {
        self.range_at(&'A', &'Z')
    }

    fn is_lower(&self) -> bool {
        self.range_at(&'a', &'z')
    }

    fn to_upper(&self) -> char {
        if self.is_lower() {
            (*self as u8).to_upper()
        } else { *self }
    }

    fn to_lower(&self) -> char {
        if self.is_upper() {
            (*self as u8).to_lower()
        } else { *self }
    }
}

impl Symbols for u8 {
    fn is_number(&self) -> bool {
        self.range_at(&b'0', &b'9')
    }

    fn is_upper(&self) -> bool {
        self.range_at(&b'A', &b'Z')
    }

    fn is_lower(&self) -> bool {
        self.range_at(&b'a', &b'z')
    }

    fn to_upper(&self) -> char {
        if self.is_lower() {
            (*self - b'a' + b'A') as char
        } else { *self as char }
    }

    fn to_lower(&self) -> char {
        if self.is_upper() {
            (*self - b'A' + b'a') as char
        } else { *self as char }
    }
}
