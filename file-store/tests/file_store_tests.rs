use file_store::{FileStore, Result};
use std::io::Write;
use tempfile::TempDir;

#[test]
fn all_uuids() {
    let dir = TempDir::new().unwrap();
    let fs = FileStore::new(dir.path()).unwrap();

    // 创建第一个更新文件并写入内容
    let (uuid, mut file) = fs.new_update().unwrap();
    file.write_all(b"Hello world").unwrap();
    file.persist().unwrap();

    // 获取所有 UUID 并进行断言
    let all_uuids = fs.all_uuids().unwrap().collect::<Result<Vec<_>>>().unwrap();
    assert_eq!(all_uuids, vec![uuid]);

    // 创建第二个更新文件
    let (uuid2, file) = fs.new_update().unwrap();
    let all_uuids = fs.all_uuids().unwrap().collect::<Result<Vec<_>>>().unwrap();
    assert_eq!(all_uuids, vec![uuid]);

    // 持久化第二个文件并再次获取所有 UUID
    file.persist().unwrap();
    let mut all_uuids = fs.all_uuids().unwrap().collect::<Result<Vec<_>>>().unwrap();
    all_uuids.sort();
    let mut expected = vec![uuid, uuid2];
    expected.sort();
    assert_eq!(all_uuids, expected);
}
