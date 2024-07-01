use arta::fs::RuntimeDirEntry;
use std::{
    ffi::OsString,
    fs::{FileType, Metadata},
    future::Future,
    path::PathBuf,
};

/// Tokio specific [`RuntimeDirEntry`] implementation.
pub struct TokioDirEntry {
    pub(super) inner: tokio::fs::DirEntry,
}

impl RuntimeDirEntry for TokioDirEntry {
    #[cfg(unix)]
    fn ino(&self) -> u64 {
        self.inner.ino()
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
        self.inner.path()
    }
}
