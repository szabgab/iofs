pub trait MetadataTime {
    fn last_access_time(&self) -> u64;
    fn last_write_time(&self) -> u64;
    fn creation_time(&self) -> u64;
}



#[cfg(windows)]
mod windows {
    use crate::prelude::DFiles;
    use std::os::windows::prelude::MetadataExt;

    use super::MetadataTime;

    impl<F: DFiles> MetadataTime for F {
        fn last_access_time(&self) -> u64 {
            if self.is_exist() {
                match self.metadata() {
                    Ok(data) => data.last_access_time(),
                    _ => 0,
                }
            } else {
                0
            }
        }

        fn last_write_time(&self) -> u64 {
            if self.is_exist() {
                match self.metadata() {
                    Ok(data) => data.last_write_time(),
                    _ => 0,
                }
            } else {
                0
            }
        }

        fn creation_time(&self) -> u64 {
            if self.is_exist() {
                match self.metadata() {
                    Ok(data) => data.creation_time(),
                    _ => 0,
                }
            } else {
                0
            }
        }
    }
}
