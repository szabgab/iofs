use super::{fd::DFiles, fd::FileDir, FileInfo};
use crate::fs::pbuilder::PathBuilder;
use crate::fs::Attributes;
use std::fs::{metadata, read_dir};
use std::{
    fmt::Debug,
    fs::{self, DirBuilder},
    io::Result,
};
#[derive(Clone)]
pub struct DirectoryInfo {
    inner: PathBuilder,
}

impl PartialEq for DirectoryInfo {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl Debug for DirectoryInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DirectoryInfo")
            .field("full_name", &self.full_name())
            .finish()
    }
}

impl DFiles for DirectoryInfo {
    fn is_exist(&self) -> bool {
        read_dir(self.full_name()).is_ok()
    }
    fn attributes(&self) -> Attributes {
        if self.is_exist() {
            Attributes::Directory
        } else {
            Attributes::None
        }
    }

    fn builder(&self) -> &PathBuilder {
        self.inner.borrow()
    }

    unsafe fn mut_builder(&mut self) -> &mut PathBuilder {
        self.inner.mut_borrow()
    }

    fn copy_new(&self, path: &str) -> Result<()> {
        let builder = PathBuilder::from(path);
        self.move_or_copy(builder.full_name(), false)?;
        Ok(())
    }

    fn move_new(&mut self, path: &str) -> Result<()> {
        let builder = PathBuilder::from(path);
        self.move_or_copy(builder.full_name(), true)?;
        fs::remove_dir_all(self.full_name())?;
        *self = unsafe { DirectoryInfo::open_uncheck(builder.full_name()) };
        Ok(())
    }
    fn size_bytes(&self) -> u64 {
        let mut size = 0;
        for f in self.files() {
            size += f.size_bytes()
        }
        for dir in self.directories() {
            size += dir.size_bytes();
        }
        size
    }
}
impl DirectoryInfo {
    pub fn open<P: AsRef<str>>(path: P) -> DirectoryInfo {
        DirectoryInfo {
            inner: PathBuilder::from(path),
        }
    }

    pub unsafe fn open_uncheck<P: AsRef<str>>(path: P) -> DirectoryInfo {
        DirectoryInfo {
            inner: PathBuilder::from_uncheck(path),
        }
    }
    pub fn open_smart<P: AsRef<str>>(path: P) -> Result<DirectoryInfo> {
        if metadata(path.as_ref()).is_err() {
            DirBuilder::new().recursive(true).create(path.as_ref())?;
        }
        Ok(DirectoryInfo::open(path))
    }
    pub fn create(&mut self) -> Result<()> {
        DirBuilder::new().recursive(true).create(self.full_name())
    }

    pub fn is_eq(&self, other: &Self) -> bool {
        fn is_eq(dir: &DirectoryInfo, other: &DirectoryInfo) -> bool {
            let self_files = dir.files();
            let other_files = other.files();
            if self_files.len() == other_files.len() {
                for i in 0..self_files.len() {
                    if self_files[i].name() != other_files[i].name()
                        || !self_files[i].is_eq(&other_files[i])
                    {
                        return false;
                    }
                }
            } else {
                return false;
            }

            let self_dirs = dir.directories();
            let other_dirs = other.directories();
            if self_dirs.len() == other_dirs.len() {
                for i in 0..self_dirs.len() {
                    if self_dirs[i].name() != other_dirs[i].name()
                        || self_dirs[i].is_eq(&other_dirs[i])
                    {
                        return false;
                    }
                }
            } else {
                return false;
            }
            true
        }
        self == other || is_eq(self, other)
    }
    fn find_children(&self, is_dir: bool, is_file: bool) -> Vec<String> {
        let child = |dir: &DirectoryInfo| -> Result<Vec<String>> {
            let mut child = Vec::new();
            let paths = read_dir(dir.full_name())?;
            for path in paths {
                let path = match path {
                    Ok(path) => path.path().display().to_string().replace("\\", "/"),
                    _ => continue,
                };
                let data = match metadata(&path) {
                    Ok(data) => data,
                    _ => continue,
                };
                if (is_dir && data.is_dir()) || (is_file && data.is_file()) {
                    child.push(path)
                }
            }
            Ok(child)
        };
        child(self).unwrap_or_default()
    }
    pub fn files(&self) -> Vec<FileInfo> {
        self.find_children(false, true)
            .into_iter()
            .map(|path| unsafe { FileInfo::open_uncheck(path) })
            .collect()
    }
    pub fn file_paths(&self) -> Vec<String> {
        self.find_children(false, true)
    }
    pub fn directories(&self) -> Vec<DirectoryInfo> {
        self.find_children(true, false)
            .into_iter()
            .map(|path| unsafe { DirectoryInfo::open_uncheck(path) })
            .collect()
    }
    pub fn directory_paths(&self) -> Vec<String> {
        self.find_children(true, false)
    }
    pub fn children(&self) -> Vec<FileDir> {
        self.find_children(true, true)
            .into_iter()
            .map(|path| unsafe { FileDir::open_uncheck(path) })
            .collect()
    }
    pub fn children_path(&self) -> Vec<String> {
        self.find_children(true, true)
    }
    pub fn contain_child(&self, child: impl AsRef<str>) -> bool {
        metadata(format!("{}/{}", self.full_name(), child.as_ref())).is_ok()
    }
    fn move_or_copy(&self, path: &str, is_move: bool) -> Result<()> {
        let mut queue = vec![self.clone()];
        while !queue.is_empty() {
            let dir = queue.remove(0);
            queue.append(&mut dir.directories());
            let full_name = dir.full_name().replace(self.full_name(), path);
            fs::create_dir_all(&full_name)?;
            for mut f in dir.files() {
                if is_move {
                    f.move_to(&full_name)?;
                } else {
                    f.copy_to(&full_name)?;
                }
            }
        }
        Ok(())
    }
    pub fn to_file_dir(self) -> FileDir {
        FileDir::from(self)
    }
}
