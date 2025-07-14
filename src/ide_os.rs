use crate::ide_os::OS::{LINUX, WINDOWS};

pub enum OS {
    WINDOWS,
    LINUX,
}
impl OS {
    pub fn get_os() -> Option<OS> {
        if cfg!(target_os = "windows") {
            Some(WINDOWS)
        } else if cfg!(target_os = "linux") {
            Some(LINUX)
        } else {
            None
        }
    }
}
