use std::{error, fs};
use std::fs::File as StdFile;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use tempfile::NamedTempFile;
use uuid::Uuid;

const UPDATE_FILES_PATH: &str = "updates/updates_files";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not parse file name as utf-8")]
    CouldNotParseFileNameAsUtf8,
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    PersistError(#[from] tempfile::PersistError),
    #[error(transparent)]
    UuidError(#[from] uuid::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct FileStore {
    path: PathBuf,
}

impl FileStore {
    pub fn new(path: impl AsRef<Path>) -> Result<FileStore> {
        let path = path.as_ref().to_path_buf();
        std::fs::create_dir_all(&path)?;
        Ok(FileStore { path })
    }
}

impl FileStore {
    pub fn new_update(&self) -> Result<(Uuid, File)> {
        let file = NamedTempFile::new_in(&self.path)?;
        let uuid = Uuid::new_v4();
        let path = self.path.join(uuid.to_string());
        let update_file = File {
            file: Some(file),
            path,
        };

        Ok((uuid, update_file))
    }

    pub fn new_update_with_uuid(&self, uuid: u128) -> Result<(Uuid, File)> {
        let file = NamedTempFile::new_in(&self.path)?;
        let uuid = Uuid::from_u128(uuid);
        let path = self.path.join(uuid.to_string());
        let update_file = File {
            file: Some(file),
            path,
        };

        Ok((uuid, update_file))
    }

    pub fn get_update_path(&self, uuid: Uuid) -> PathBuf {
        self.path.join(uuid.to_string())
    }

    pub fn get_update(&self, uuid: Uuid) -> Result<StdFile> {
        let path = self.get_update_path(uuid);
        let file = match StdFile::open(path) {
            Ok(file) => file,
            Err(e) => {
                tracing::error!("Can't access update file {uuid}: {e}");
                return Err(e.into());
            }
        };
        Ok(file)
    }

    pub fn snapshot(&self, uuid: Uuid, dst: impl AsRef<Path>) -> Result<()> {
        let src = self.path.join(uuid.to_string());
        let mut dst = dst.as_ref().join(UPDATE_FILES_PATH);
        std::fs::create_dir_all(&dst)?;
        dst.push(uuid.to_string());
        std::fs::copy(src, dst)?;
        Ok(())
    }
}
