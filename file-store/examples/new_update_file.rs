use std::path::Path;
use tempfile::TempDir;
use uuid::Uuid;
use your_crate::FileStore; // 替换为实际的 crate 名称

fn main() -> file_store::Result<()> {
    // 创建一个临时目录作为文件存储路径
    let temp_dir = TempDir::new()?;
    println!("临时目录路径: {:?}", temp_dir.path());
    let file_store = FileStore::new(temp_dir.path())?;

    // 创建一个新的更新文件
    let (uuid, mut file) = file_store.new_update()?;

    // 写入一些数据到文件中
    file.write_all(b"Hello, world!")?;

    // 持久化文件
    file.persist()?;

    println!("Created new update file with UUID: {}", uuid);
}
