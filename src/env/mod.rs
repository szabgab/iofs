use crate::fs::pathstr::PathStr;
use std::env::{self, VarError};
use std::io;

pub struct EnVal;

impl EnVal {
    pub fn home() -> Result<String, VarError> {
        if cfg!(windows) {
            let drive_key = "HOMEDRIVE";
            let home_key = "HOMEPATH";
            let Ok(drive) = env::var(drive_key) else { return Err(VarError::NotPresent) };
            let Ok(home) = env::var(home_key) else { return Err(VarError::NotPresent) };
            Ok(format!("{}{}", drive, home).replace("\\", "/"))
        } else if cfg!(unix) {
            let key = "HOME";
            env::var(key)
        } else {
            Err(VarError::NotPresent)
        }
    }
}

pub fn current_dir() -> io::Result<String> {
    let path = env::current_dir()?.display().to_string();
    Ok(path.correct())
}
pub fn current_program() -> io::Result<String> {
    let path = env::current_exe()?.display().to_string();
    Ok(path.correct())
}
