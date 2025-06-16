use std::fs::{self, File};
use tempfile::{TempDir, tempdir};

pub fn create_test_directory() -> TempDir {
  let temp_dir = tempdir().unwrap();
  fs::create_dir(temp_dir.path().join("subdir")).unwrap();
  fs::create_dir(temp_dir.path().join(".hidden_dir")).unwrap();
  File::create(temp_dir.path().join("file1.txt")).unwrap();
  File::create(temp_dir.path().join("file2.rs")).unwrap();
  File::create(temp_dir.path().join(".hidden_file")).unwrap();
  File::create(temp_dir.path().join("subdir/file3.txt")).unwrap();
  File::create(temp_dir.path().join("subdir/file4.md")).unwrap();
  temp_dir
}
