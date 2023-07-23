use std::fs::File;

use crate::{
    env,
    prelude::{range::Range, symbols::Symbols, Find},
};

use super::pbuilder::{PathBuilder, UriKind};

pub trait PathStr {
    fn uri_kind(&self) -> UriKind;
    fn correct(&self) -> String;
    fn relate_to_absolute(&self) -> String;
    fn is_root(&self) -> bool;
}

impl<P: AsRef<str>> PathStr for P {
    fn uri_kind(&self) -> UriKind {
        if cfg!(windows) {
            let bytes = self.as_ref().as_bytes();
            if bytes.len() > 1 && bytes[0].is_letter() && bytes[1] == b':' {
                UriKind::Absolute
            } else {
                UriKind::Relative
            }
        } else {
            if self.as_ref().starts_with("/") {
                UriKind::Absolute
            } else {
                UriKind::Relative
            }
        }
    }

    fn correct(&self) -> String {
        let mut buf = String::new();
        for char in self.as_ref().chars() {
            if char.contained_in(&['\\', '/']) {
                if !buf.ends_with("/") {
                    buf.push('/')
                }
            } else if char != '?' {
                // unstable
                buf.push(char)
            }
        }
        if cfg!(windows) {
            if buf.ends_with("/") {
                if buf.len() == 2 || buf.len() == 3 {
                } else {
                    buf.pop();
                }
            }
            if buf.starts_with("/") {
                buf.remove(0);
            }
        } else {
            if buf.len() > 1 && buf.ends_with("/") {
                buf.pop();
            }
        }
        buf
    }

    fn relate_to_absolute(&self) -> String {
        match env::current_dir() {
            Ok(dir) => format!("{}/{}", dir, self.as_ref()).correct(),
            _ => self.as_ref().to_owned(),
        }
    }

    fn is_root(&self) -> bool {
        let path = self.correct();
        if cfg!(windows) {
            let bytes = path.as_bytes();
            (bytes.len() >= 2 && bytes[0].is_letter() && bytes[1] == b':')
                && (bytes.len() == 2 || (bytes.len() == 3 && bytes[2] == b'/'))
        } else {
            path.as_str() == "/"
        }
    }
}

pub trait FileString {
    fn to_string(self) -> String;
}

impl FileString for File {
    fn to_string(self) -> String {
        let file = format!("{:?}", self);
        match file.as_bytes().find_all(&b'\"') {
            Some(pos) => {
                let path = &file.as_str()[pos[0] + 1..pos[1]];
                let path = PathBuilder::from(path);
                path.string_clone()
            }
            _ => String::new(),
        }
    }
}
