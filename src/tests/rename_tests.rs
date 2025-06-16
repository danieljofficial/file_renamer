// use super::helpers::create_test_directory;
// use crate::PatternRename;
#[cfg(test)]

pub mod tests {
  use std::fs::File;

  use crate::{PatternRename, core::rename::RenameStrategy, tests::helpers::create_test_directory};

  #[test]
  fn test_pattern_rename_basic() {
    let temp_dir = create_test_directory();
    let original = temp_dir.path().join("file1.txt");
    let mut renamer = PatternRename::new("{name}_backup.{ext}".to_string()).unwrap();

    println!("extension: {:#?}", original.extension());
    let new_path = renamer.generate_name(&original).unwrap();

    assert_eq!(new_path, temp_dir.path().join("file1_backup.txt"))
  }

  #[test]
  fn test_pattern_rename_with_parent() {
    let temp_dir = create_test_directory();
    let original = temp_dir.path().join("subdir/file4.md");
    let mut renamer = PatternRename::new("{parent}_{name}.{ext}".to_string()).unwrap();

    let new_path = renamer.generate_name(&original).unwrap();

    assert_eq!(new_path, temp_dir.path().join("subdir/subdir_file4.md"))
  }

  #[test]
  fn test_pattern_rename_with_index() {
    let temp_dir = create_test_directory();
    let files = vec![
      temp_dir.path().join("file1.txt"),
      temp_dir.path().join("file2.rs"),
    ];
    let mut renamer = PatternRename::new("{name}_{i}.{ext}".to_string()).unwrap();
    let first = renamer.generate_name(&files[0]).unwrap();
    let second = renamer.generate_name(&files[1]).unwrap();

    assert_eq!(first, temp_dir.path().join("file1_1.txt"));
    assert_eq!(second, temp_dir.path().join("file2_2.rs"));
  }

  #[test]
  fn test_pattern_rename_no_extension() {
    let temp_dir = create_test_directory();
    let original = temp_dir.path().join("README");
    File::create(&original).unwrap();

    let mut renamer = PatternRename::new("{name}_v1{ext}".to_string()).unwrap();
    let new_path = renamer.generate_name(&original).unwrap();

    assert_eq!(new_path, temp_dir.path().join("README_v1"))
  }

  #[test]
  fn test_pattern_rename_invalid_pattern() {
    assert!(PatternRename::new("pattern".to_string()).is_err());
    assert!(PatternRename::new("{name}:".to_string()).is_err());
    assert!(PatternRename::new("../{name}".to_string()).is_err());
    assert!(PatternRename::new("/absolute/{name}".to_string()).is_err());
  }

  #[test]
  fn test_pattern_rename_with_date_placeholder() {
    let temp_dir = create_test_directory();
    let original = temp_dir.path().join("file1.txt");
    let mut renamer = PatternRename::new("{name}_{date}.{ext}".to_string()).unwrap();
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let expected = format!("file1_{}.txt", date);
    let new_path = renamer.generate_name(&original).unwrap();
    println!("new path: {:#?}, expected: {:#?}", &new_path, &expected);

    assert_eq!(new_path, temp_dir.path().join(expected));
  }

  #[test]
  fn test_pattern_rename_multiple_placeholders() {
    let temp_dir = create_test_directory();
    let original = temp_dir.path().join("subdir/file3.txt");
    let mut renamer =
      PatternRename::new("backup_{parent}_{name}_{i}_{date}.{ext}".to_string()).unwrap();
    let new_path = renamer.generate_name(&original).unwrap();
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let expected = format!("backup_subdir_file3_1_{}.txt", date);

    assert_eq!(new_path, temp_dir.path().join("subdir/").join(expected))
  }
}
