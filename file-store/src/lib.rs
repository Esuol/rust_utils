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

    pub fn compute_size(&self, uuid: Uuid) -> Result<u64> {
        Ok(self.get_update(uuid)?.metadata()?.len())
    }

    pub fn delete(&self, uuid: Uuid) -> Result<()> {
        let path = self.path.join(uuid.to_string());
        if let Err(e) = std::fs::remove_file(path) {
            tracing::error!("Can't delete file {uuid}: {e}");
            Err(e.into())
        } else {
            Ok(())
        }
    }

    pub fn all_uuids(&self) -> Result<impl Iterator<Item = Result<Uuid>>> {
        Ok(self.path.read_dir()?.filter_map(|entry| {
            let file_name = match entry {
                Ok(entry) => entry.file_name(),
                Err(e) => return Some(Err(e.into())),
            };
            let file_name = match file_name.to_str() {
                Some(file_name) => file_name,
                None => return Some(Err(Error::CouldNotParseFileNameAsUtf8)),
            };
            if file_name.starts_with('.') {
                None
            } else {
                Some(Uuid::from_str(file_name).map_err(|e| e.into()))
            }
        }))
    }
}

pub struct File {
    path: PathBuf,
    file: Option<NamedTempFile>,
}

impl File {
    pub fn dry_file() -> Result<Self> {
        Ok(Self {
            path: PathBuf::new(),
            file: None,
        })
    }

    pub fn persist(self) -> Result<()> {
        if let Some(file) = self.file {
            file.persist(&self.path)?;
            println!("Persisted file to {:?}", self.path);
        }
        Ok(())
    }
}

impl Write for File {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if let Some(file) = self.file.as_mut() {
            file.write(buf)
        } else {
            Ok(buf.len())
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if let Some(file) = self.file.as_mut() {
            file.flush()
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use std::io::Write;

    use tempfile::TempDir;

    use super::*;

    #[test]
    fn all_uuids() {
        let dir = TempDir::new().unwrap();
        let fs = FileStore::new(dir.path()).unwrap();
        let (uuid, mut file) = fs.new_update().unwrap();
        file.write_all(b"Hello world").unwrap();
        file.persist().unwrap();
        let all_uuids = fs.all_uuids().unwrap().collect::<Result<Vec<_>>>().unwrap();
        assert_eq!(all_uuids, vec![uuid]);

        let (uuid2, file) = fs.new_update().unwrap();
        let all_uuids = fs.all_uuids().unwrap().collect::<Result<Vec<_>>>().unwrap();
        assert_eq!(all_uuids, vec![uuid]);

        file.persist().unwrap();
        let mut all_uuids = fs.all_uuids().unwrap().collect::<Result<Vec<_>>>().unwrap();
        all_uuids.sort();
        let mut expected = vec![uuid, uuid2];
        expected.sort();
        assert_eq!(all_uuids, expected);
    }
}
