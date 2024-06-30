use std::ffi::OsString;
use std::fs::{FileType, Metadata};
use std::future::Future;
use std::path::PathBuf;

pub trait RuntimeDirEntry {
    #[cfg(any(target_family = "unix", doc))]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    fn ino(&self) -> u64;

    fn file_name(&self) -> OsString;

    fn file_type(&self) -> impl Future<Output = std::io::Result<FileType>> + Send;

    fn metadata(&self) -> impl Future<Output = std::io::Result<Metadata>> + Send;

    fn path(&self) -> PathBuf;
}
