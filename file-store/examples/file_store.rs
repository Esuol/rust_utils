use file_store::{FileStore, Result};
use std::io::Write;
use tempfile::TempDir;

fn main() -> Result<()> {
    let dir = TempDir::new()?;
    let fs = FileStore::new(dir.path())?;

    // 创建第一个更新文件并写入内容
    let (uuid, mut file) = fs.new_update()?;
    file.write_all(b"Hello world")?;
    file.persist()?;

    // 获取所有 UUID 并进行断言
    let all_uuids = fs.all_uuids()?.collect::<Result<Vec<_>>>()?;
    assert_eq!(all_uuids, vec![uuid]);

    // 创建第二个更新文件
    let (uuid2, file) = fs.new_update()?;
    let all_uuids = fs.all_uuids()?.collect::<Result<Vec<_>>>()?;
    assert_eq!(all_uuids, vec![uuid]);

    // 持久化第二个文件并再次获取所有 UUID
    file.persist()?;
    let mut all_uuids = fs.all_uuids()?.collect::<Result<Vec<_>>>()?;
    all_uuids.sort();
    let mut expected = vec![uuid, uuid2];
    expected.sort();
    assert_eq!(all_uuids, expected);

    Ok(())
}
