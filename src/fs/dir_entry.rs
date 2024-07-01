use std::ffi::OsString;
use std::fs::{FileType, Metadata};
use std::future::Future;
use std::path::PathBuf;

/// An instance of `RuntimeDirEntry` represents an entry inside of a directory on the
/// filesystem. Each entry can be inspected via methods to learn about the full
/// path or possibly other metadata through per-platform extension traits.
pub trait RuntimeDirEntry {
    /// Returns the underlying `d_ino` field in the contained `dirent`
    /// structure.
    #[cfg(any(target_family = "unix", doc))]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    fn ino(&self) -> u64;

    /// Returns the file name of this directory entry without any
    /// leading path component(s).
    fn file_name(&self) -> OsString;

    /// Returns the file type for the file that this entry points at.
    ///
    /// This function will not traverse symlinks if this entry points at a
    /// symlink.
    fn file_type(&self) -> impl Future<Output = std::io::Result<FileType>> + Send;

    /// Returns the metadata for the file that this entry points at.
    fn metadata(&self) -> impl Future<Output = std::io::Result<Metadata>> + Send;

    /// Returns the full path to the file that this entry represents.
    fn path(&self) -> PathBuf;
}
