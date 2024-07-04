use arta::fs::RuntimeDirEntry;
use futures::prelude::Future;
use std::{
    ffi::OsString,
    fs::{FileType, Metadata},
    path::PathBuf,
};

/// Async-std specific [`RuntimeDirEntry`] implementation.
pub struct AsyncStdDirEntry {
    pub(super) inner: async_std::fs::DirEntry,
}

impl RuntimeDirEntry for AsyncStdDirEntry {
    #[cfg(unix)]
    fn ino(&self) -> u64 {
        async_std::os::unix::fs::DirEntryExt::ino(&self.inner)
    }

    fn file_name(&self) -> OsString {
        self.inner.file_name()
    }

    fn file_type(&self) -> impl Future<Output = std::io::Result<FileType>> + Send {
        self.inner.file_type()
    }

    fn metadata(&self) -> impl Future<Output = std::io::Result<Metadata>> + Send {
        self.inner.metadata()
    }

    fn path(&self) -> PathBuf {
        self.inner.path().into()
    }
}
