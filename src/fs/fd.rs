use super::{DirectoryInfo, FileInfo};
use crate::fs::pbuilder::PathBuilder;
use std::fs::{self, metadata, Metadata};
use std::io::{Error, ErrorKind, Result};
use std::time::SystemTime;

pub trait DFiles {
    fn name(&self) -> &str {
        self.builder().name()
    }

    /// Return the name without the extension
    /// # Example
    /// ```rust
    /// let f = FileInfo::open("foo.txt");
    /// assert_eq!("foo", f.ext_name())
    /// ```
    ///
    fn ext_name(&self) -> &str {
        self.builder().ext_name()
    }
    fn full_name(&self) -> &str {
        self.builder().full_name()
    }
    fn is_exist(&self) -> bool;
    fn attributes(&self) -> Attributes;

    fn is_hide(&self) -> bool {
        self.name().starts_with(".")
    }
    fn rename(&mut self, new_name: &str) -> Result<()> {
        let parent = self.parent_str();
        let extension = self.extension();
        fs::rename(
            format!("{}/{}{}", parent, self.ext_name(), extension),
            format!("{}/{}{}", parent, new_name, extension),
        )?;
        unsafe {
            self.mut_builder().rename(new_name);
        }
        Ok(())
    }
    fn extension(&self) -> &str {
        self.builder().extension()
    }
    fn extension_match(&self, needle: &[&str]) -> bool {
        let extension = self.extension();
        for pat in needle {
            if *pat == extension {
                return true;
            }
        }
        false
    }
    fn extension_match_ignore_ascii_case(&self, needle: &[&str]) -> bool {
        let extension = self.extension().to_ascii_lowercase();
        for pat in needle {
            if pat.to_ascii_lowercase() == extension {
                return true;
            }
        }
        false
    }

    fn set_extension(&mut self, exs: &str) -> Result<()> {
        unsafe {
            let exs = if exs.starts_with(".") {
                exs.to_owned()
            } else {
                format!(".{}", exs)
            };
            fs::rename(
                self.full_name(),
                format!("{}/{}{}", self.parent_str(), self.ext_name(), exs),
            )?;
            self.mut_builder().set_extension(exs.as_str());
        }
        Ok(())
    }
    fn del(&self) -> Result<()> {
        match self.attributes() {
            Attributes::File => fs::remove_file(self.full_name()),
            Attributes::Directory => fs::remove_dir_all(self.full_name()),
            _ => Ok(()),
        }
    }
    fn builder(&self) -> &PathBuilder;
    unsafe fn mut_builder(&mut self) -> &mut PathBuilder;
    fn copy_to(&self, path: &str) -> Result<()> {
        self.copy_new(&format!("{}/{}", path, self.name()))
    }
    fn copy_new(&self, path: &str) -> Result<()>;
    fn move_to(&mut self, path: &str) -> Result<()> {
        self.move_new(&format!("{}/{}", path, self.name()))
    }
    fn move_new(&mut self, path: &str) -> Result<()>;
    fn cover_to(&mut self, path: &str, is_move: bool) -> Result<()> {
        self.cover_new(format!("{}/{}", path, self.name()).as_str(), is_move)
    }
    fn cover_new(&mut self, path: &str, is_move: bool) -> Result<()> {
        let builder = PathBuilder::from(path);
        if builder.full_name() == self.full_name() {
            return Ok(());
        }
        check(builder.full_name())?;
        if is_move {
            self.move_new(builder.full_name())
        } else {
            self.copy_new(builder.full_name())
        }
    }

    fn metadata(&self) -> Result<Metadata> {
        metadata(self.full_name())
    }
    fn modified(&self) -> Result<SystemTime> {
        if self.is_exist() {
            self.metadata()?.modified()
        } else {
            Err(not_found_err())
        }
    }
    fn size_bytes(&self) -> u64;
    fn size_kb(&self) -> u64 {
        self.size_bytes() / 1024
    }
    fn size_mb(&self) -> u64 {
        self.size_kb() / 1024
    }
    fn is_read_only(&self) -> bool {
        match metadata(self.full_name()) {
            Ok(data) => data.permissions().readonly(),
            _ => false,
        }
    }
    fn parent_str(&self) -> &str {
        self.builder().parent()
    }
    fn parent(&self) -> DirectoryInfo {
        DirectoryInfo::open(self.parent_str())
    }
}

fn check<P: AsRef<str>>(path: P) -> Result<()> {
    match metadata(path.as_ref()) {
        Ok(data) => {
            if data.is_dir() {
                fs::remove_dir_all(path.as_ref())
            } else {
                fs::remove_file(path.as_ref())
            }
        }
        _ => Ok(()),
    }
}

/// None != None
pub enum Attributes {
    File,
    Directory,
    None,
}

impl Attributes {
    pub fn to_i32(&self) -> i32 {
        match self {
            Attributes::File => 0,
            Attributes::Directory => 1,
            Attributes::None => -1,
        }
    }
}
impl PartialEq for Attributes {
    fn eq(&self, other: &Self) -> bool {
        let n = self.to_i32();
        n != -1 && n == other.to_i32()
    }
}
pub struct FileDir {
    inner: Box<dyn DFiles>,
}

impl From<FileInfo> for FileDir {
    fn from(value: FileInfo) -> Self {
        Self {
            inner: Box::new(value),
        }
    }
}
impl From<DirectoryInfo> for FileDir {
    fn from(value: DirectoryInfo) -> Self {
        Self {
            inner: Box::new(value),
        }
    }
}

impl From<Box<dyn DFiles>> for FileDir {
    fn from(value: Box<dyn DFiles>) -> Self {
        FileDir { inner: value }
    }
}

impl DFiles for FileDir {
    fn is_exist(&self) -> bool {
        self.inner.is_exist()
    }

    fn attributes(&self) -> Attributes {
        self.inner.attributes()
    }

    fn builder(&self) -> &PathBuilder {
        self.inner.builder()
    }

    unsafe fn mut_builder(&mut self) -> &mut PathBuilder {
        self.inner.mut_builder()
    }

    fn copy_new(&self, path: &str) -> Result<()> {
        self.inner.copy_new(path)
    }

    fn move_new(&mut self, path: &str) -> Result<()> {
        self.inner.move_new(path)
    }

    fn size_bytes(&self) -> u64 {
        self.inner.size_bytes()
    }
}

impl FileDir {
    pub fn open(path: &str) -> Result<FileDir> {
        let inner = to_file_dir(path)?;
        Ok(FileDir { inner })
    }
    pub unsafe fn open_uncheck<P: AsRef<str>>(path: P) -> FileDir {
        let inner = match metadata(path.as_ref()) {
            Ok(data) => {
                if data.is_dir() {
                    Box::new(DirectoryInfo::open_uncheck(path))
                } else {
                    Box::new(FileInfo::open_uncheck(path)) as Box<dyn DFiles>
                }
            }
            _ => panic!("Cannot find the specified file or directory!"),
        };
        FileDir { inner }
    }

    fn to_file(&self) -> FileInfo {
        unsafe { FileInfo::open_uncheck(self.full_name()) }
    }
    fn to_dir(&self) -> DirectoryInfo {
        unsafe { DirectoryInfo::open_uncheck(self.full_name()) }
    }
    pub fn is_eq(&self, other: &Self) -> bool {
        match self.common_attr_with(other) {
            Attributes::File => self.to_file().is_eq(&other.to_file()),
            Attributes::Directory => self.to_dir().is_eq(&other.to_dir()),
            _ => false,
        }
    }

    pub fn is_dir(&self) -> bool {
        match self.metadata() {
            Ok(data) => data.is_dir(),
            _ => false,
        }
    }
    pub fn is_file(&self) -> bool {
        match self.metadata() {
            Ok(data) => data.is_file(),
            _ => false,
        }
    }

    pub fn common_attr_with(&self, other: &Self) -> Attributes {
        let attributes = self.attributes();
        if attributes == other.attributes() {
            attributes
        } else {
            Attributes::None
        }
    }
    pub fn extension(&self) -> &str {
        self.builder().extension()
    }
    pub fn attr_eq(&self, other: &Self) -> bool {
        match self.common_attr_with(other) {
            Attributes::None => false,
            _ => true,
        }
    }
    pub fn children(&self) -> Option<Vec<FileDir>> {
        if self.is_dir() {
            Some(self.to_dir().children())
        } else {
            None
        }
    }
    pub fn files(&self) -> Option<Vec<FileInfo>> {
        if self.is_dir() {
            Some(self.to_dir().files())
        } else {
            None
        }
    }

    pub fn directories(&self) -> Option<Vec<DirectoryInfo>> {
        if self.is_dir() {
            Some(self.to_dir().directories())
        } else {
            None
        }
    }
}

fn to_file_dir(path: &str) -> Result<Box<dyn DFiles>> {
    let data = metadata(path)?;
    if data.is_dir() {
        Ok(Box::new(DirectoryInfo::open(path)))
    } else {
        let f = FileInfo::open(path);
        Ok(Box::new(f))
    }
}
#[allow(unused)]
fn already_exists_err() -> Error {
    Error::new(
        ErrorKind::AlreadyExists,
        "The file or directory already exists!",
    )
}
#[allow(unused)]
fn not_found_err() -> Error {
    Error::new(ErrorKind::NotFound, "The file or directory is not found!")
}
