pub mod helpers;
pub mod list_paths_tests;
pub mod rename_tests;

// #[cfg(test)]
// pub mod tests {
// use crate::{
//   PatternRename,
//   core::{ListOptions, list_paths, rename::RenameStrategy},
// };
// use std::fs::{self, File};
// use tempfile::{TempDir, tempdir};

// fn create_test_directory() -> TempDir {
//   let temp_dir = tempdir().unwrap();
//   fs::create_dir(temp_dir.path().join("subdir")).unwrap();
//   fs::create_dir(temp_dir.path().join(".hidden_dir")).unwrap();
//   File::create(temp_dir.path().join("file1.txt")).unwrap();
//   File::create(temp_dir.path().join("file2.rs")).unwrap();
//   File::create(temp_dir.path().join(".hidden_file")).unwrap();
//   File::create(temp_dir.path().join("subdir/file3.txt")).unwrap();
//   File::create(temp_dir.path().join("subdir/file4.md")).unwrap();
//   temp_dir
// }
// #[test]
// fn test_list_files_only() {
//   let temp_dir = create_test_directory();
//   let options = ListOptions {
//     include_directories: false,
//     ..Default::default()
//   };
//   let result = list_paths(temp_dir.path().to_str().unwrap(), &options).unwrap();
//   assert_eq!(result.len(), 2);
//   assert!(result.iter().any(|p| p.ends_with("file1.txt")));
//   assert!(result.iter().any(|p| p.ends_with("file2.rs")));
//   assert!(!result.iter().any(|p| p.ends_with("subdir")));
// }
// #[test]
// fn test_list_files_and_directories() {
//   let temp_dir = create_test_directory();
//   let options = ListOptions {
//     include_directories: true,
//     ..Default::default()
//   };

//   let result = list_paths(temp_dir.path().to_str().unwrap(), &options).unwrap();

//   assert_eq!(result.len(), 3);
//   assert!(result.iter().any(|p| p.ends_with("file1.txt")));
//   assert!(result.iter().any(|p| p.ends_with("file2.rs")));
//   assert!(result.iter().any(|p| p.ends_with("subdir")));
// }
// #[test]
// fn test_list_recursive_with_directories() {
//   let temp_dir = create_test_directory();
//   let options = ListOptions {
//     recursive: true,
//     include_directories: true,
//     ..Default::default()
//   };
//   let result = list_paths(temp_dir.path().to_str().unwrap(), &options).unwrap();
//   println!("Result: {:?}", result);
//   assert_eq!(result.len(), 5);
//   assert!(result.iter().any(|p| p.ends_with("file1.txt")));
//   assert!(result.iter().any(|p| p.ends_with("file2.rs")));
//   assert!(result.iter().any(|p| p.ends_with("subdir")));
//   assert!(result.iter().any(|p| p.ends_with("subdir/file3.txt")));
//   assert!(result.iter().any(|p| p.ends_with("subdir/file4.md")));
// }
// #[test]
// fn test_list_hidden_files() {
//   let temp_dir = create_test_directory();
//   let options = ListOptions {
//     include_hidden: true,
//     include_directories: false,
//     ..Default::default()
//   };
//   let result = list_paths(temp_dir.path().to_str().unwrap(), &options).unwrap();
//   assert!(!result.iter().any(|p| p.ends_with(".hidden_dir")));
//   assert!(result.iter().any(|p| p.ends_with(".hidden_file")));
// }
// #[test]
// fn test_list_with_extension_filter() {
//   let temp_dir = create_test_directory();
//   let options = ListOptions {
//     extensions: Some(vec!["txt".to_string()]),
//     recursive: true,
//     include_directories: false,
//     ..Default::default()
//   };
//   let result = list_paths(temp_dir.path().to_str().unwrap(), &options).unwrap();
//   assert_eq!(result.len(), 2);
//   assert!(result.iter().any(|p| p.ends_with("file1.txt")));
//   assert!(!result.iter().any(|p| p.ends_with("file2.rs")));
//   assert!(!result.iter().any(|p| p.ends_with("subdir")));
// }
// #[test]
// fn test_list_paths_all_filters() {
//   let temp_dir = create_test_directory();
//   let options = ListOptions {
//     recursive: true,
//     include_hidden: true,
//     include_directories: true,
//     extensions: Some(vec!["txt".to_string()]),
//   };
//   let result = list_paths(temp_dir.path().to_str().unwrap(), &options).unwrap();
//   assert!(result.iter().any(|p| p.ends_with("file1.txt")));
//   assert!(!result.iter().any(|p| p.ends_with("file2.rs")));
//   assert!(result.iter().any(|p| p.ends_with("subdir/file3.txt")));
//   assert!(!result.iter().any(|p| p.ends_with("subdir/file4.md")));
//   assert!(result.iter().any(|p| p.ends_with(".hidden_dir")));
// }

// #[test]
// fn test_pattern_rename_basic() {
//   let temp_dir = create_test_directory();
//   let original = temp_dir.path().join("file1.txt");
//   let mut renamer = PatternRename::new("{name}_backup.{ext}".to_string()).unwrap();

//   println!("extension: {:#?}", original.extension());
//   let new_path = renamer.generate_name(&original).unwrap();

//   assert_eq!(new_path, temp_dir.path().join("file1_backup.txt"))
// }

// #[test]
// fn test_pattern_rename_with_parent() {
//   let temp_dir = create_test_directory();
//   let original = temp_dir.path().join("subdir/file4.md");
//   let mut renamer = PatternRename::new("{parent}_{name}.{ext}".to_string()).unwrap();

//   let new_path = renamer.generate_name(&original).unwrap();

//   assert_eq!(new_path, temp_dir.path().join("subdir/subdir_file4.md"))
// }

// #[test]
// fn test_pattern_rename_with_index() {
//   let temp_dir = create_test_directory();
//   let files = vec![
//     temp_dir.path().join("file1.txt"),
//     temp_dir.path().join("file2.rs"),
//   ];
//   let mut renamer = PatternRename::new("{name}_{i}.{ext}".to_string()).unwrap();
//   let first = renamer.generate_name(&files[0]).unwrap();
//   let second = renamer.generate_name(&files[1]).unwrap();

//   assert_eq!(first, temp_dir.path().join("file1_1.txt"));
//   assert_eq!(second, temp_dir.path().join("file2_2.rs"));
// }

// #[test]
// fn test_pattern_rename_no_extension() {
//   let temp_dir = create_test_directory();
//   let original = temp_dir.path().join("README");
//   File::create(&original).unwrap();

//   let mut renamer = PatternRename::new("{name}_v1{ext}".to_string()).unwrap();
//   let new_path = renamer.generate_name(&original).unwrap();

//   assert_eq!(new_path, temp_dir.path().join("README_v1"))
// }

// #[test]
// fn test_pattern_rename_invalid_pattern() {
//   assert!(PatternRename::new("pattern".to_string()).is_err());
//   assert!(PatternRename::new("{name}:".to_string()).is_err());
//   assert!(PatternRename::new("../{name}".to_string()).is_err());
//   assert!(PatternRename::new("/absolute/{name}".to_string()).is_err());
// }

// #[test]
// fn test_pattern_rename_with_date_placeholder() {
//   let temp_dir = create_test_directory();
//   let original = temp_dir.path().join("file1.txt");
//   let mut renamer = PatternRename::new("{name}_{date}.{ext}".to_string()).unwrap();
//   let date = chrono::Local::now().format("%Y-%m-%d").to_string();
//   let expected = format!("file1_{}.txt", date);
//   let new_path = renamer.generate_name(&original).unwrap();
//   println!("new path: {:#?}, expected: {:#?}", &new_path, &expected);

//   assert_eq!(new_path, temp_dir.path().join(expected));
// }

// #[test]
// fn test_pattern_rename_multiple_placeholders() {
//   let temp_dir = create_test_directory();
//   let original = temp_dir.path().join("subdir/file3.txt");
//   let mut renamer =
//     PatternRename::new("backup_{parent}_{name}_{i}_{date}.{ext}".to_string()).unwrap();
//   let new_path = renamer.generate_name(&original).unwrap();
//   let date = chrono::Local::now().format("%Y-%m-%d").to_string();
//   let expected = format!("backup_subdir_file3_1_{}.txt", date);

//   assert_eq!(new_path, temp_dir.path().join("subdir/").join(expected))
// }
// }
