use crate::fs::pathstr::PathStr;
use crate::prelude::Find;
use std::fmt::{Display, Formatter};
use std::fs::metadata;

#[derive(Debug)]
pub enum UriKind {
    Absolute,
    Relative,
}

#[derive(PartialEq, Clone, Debug)]
pub struct PathBuilder {
    inner: String,
}
impl Display for PathBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.full_name())
    }
}

impl<P: AsRef<str>> From<P> for PathBuilder {
    fn from(value: P) -> Self {
        let path = value.as_ref().correct();
        let path = match path.uri_kind() {
            UriKind::Absolute => path,
            _ => value.as_ref().relate_to_absolute().correct(),
        };
        PathBuilder { inner: path }
    }
}

impl PathBuilder {
    pub unsafe fn from_uncheck<P: AsRef<str>>(path: P) -> PathBuilder {
        PathBuilder {
            inner: path.as_ref().to_owned(),
        }
    }
    pub fn borrow(&self) -> &PathBuilder {
        self
    }
    pub fn is_exist(&self) -> bool {
        metadata(self.full_name()).is_ok()
    }

    pub unsafe fn mut_borrow(&mut self) -> &mut PathBuilder {
        self
    }
    pub fn string_clone(&self) -> String {
        self.inner.clone()
    }
    pub fn replace(&mut self, from: &str, to: &str) {
        self.inner = self.inner.replace(from, to);
    }
    pub fn full_name(&self) -> &str {
        self.inner.as_str()
    }
    pub fn ext_name(&self) -> &str {
        let name = self.name();
        match name.as_bytes().find_last(&b'.') {
            Some(k) if k != 0 => &name[..k],
            _ => name,
        }
    }
    pub fn name(&self) -> &str {
        match self.last_pos(b'/') {
            Some(k) if k != 0 => &self.inner.as_str()[k + 1..],
            _ => "",
        }
    }
    pub fn rename<P: AsRef<str>>(&mut self, name: P) {
        let pos = self.last_pos(b'/').expect("rename");
        let extension = self.extension().to_string();
        unsafe {
            self.inner.as_mut_vec().set_len(pos + 1);
            self.inner
                .push_str(format!("{}{}", name.as_ref(), extension).as_str())
        }
    }
    pub fn push(&mut self, ch: char) {
        self.inner.push(ch)
    }
    pub fn push_byte(&mut self, byte: u8) {
        unsafe { self.inner.as_mut_vec().push(byte) }
    }
    pub fn push_str(&mut self, string: &str) {
        self.inner.push_str(string)
    }
    pub fn parent(&self) -> &str {
        if self.inner.is_root() {
            "%Root%"
        } else {
            unsafe {
                let k = self.inner.as_bytes().find_last(&b'/').expect("Parent");
                self.inner.get_unchecked(..k)
            }
        }
    }
    pub fn set_parent(&mut self, parent: &str) {
        self.inner = format!("{}/{}", parent, self.ext_name()).correct();
    }
    pub fn is_hide(&self) -> bool {
        self.ext_name().starts_with(".")
    }
    pub fn set_hide(&mut self, hide: bool) {
        let mut name = self.ext_name().to_string();
        if hide {
            if !self.is_hide() {
                unsafe {
                    name.as_mut_vec().insert(0, b'.');
                }
                self.rename(name)
            }
        } else {
            if self.is_hide() {
                unsafe {
                    name.as_mut_vec().remove(0);
                }
                self.rename(name.as_str())
            }
        }
    }

    pub fn set_extension(&mut self, extension: &str) {
        match self.last_pos(b'.') {
            Some(k) => unsafe { self.inner.as_mut_vec().set_len(k) },
            _ => (),
        }
        if !extension.starts_with(".") {
            self.inner.push('.')
        }

        self.inner.push_str(extension);
    }
    pub fn extension(&self) -> &str {
        let ext_name = self.name();
        match ext_name.as_bytes().find_last(&b'.') {
            Some(k) if k != 0 => &ext_name[k..],
            _ => "",
        }
    }
    fn last_pos(&self, byte: u8) -> Option<usize> {
        self.inner.as_bytes().find_last(&byte)
    }
}
