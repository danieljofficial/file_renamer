#[cfg(test)]
pub mod tests {
  use crate::{ListOptions, core::list_paths, tests::helpers::create_test_directory};

  // use super::helpers::create_test_directory;
  // use crate::core::{ListOptions, list_paths};
  // use crate::tests::helpers::create_test_directory;
  #[test]
  fn test_list_files_only() {
    let temp_dir = create_test_directory();
    let options = ListOptions {
      include_directories: false,
      ..Default::default()
    };
    let result: Vec<std::path::PathBuf> =
      list_paths(temp_dir.path().to_str().unwrap(), &options).unwrap();
    assert_eq!(result.len(), 2);
    assert!(result.iter().any(|p| p.ends_with("file1.txt")));
    assert!(result.iter().any(|p| p.ends_with("file2.rs")));
    assert!(!result.iter().any(|p| p.ends_with("subdir")));
  }
  #[test]
  fn test_list_files_and_directories() {
    let temp_dir = create_test_directory();
    let options = ListOptions {
      include_directories: true,
      ..Default::default()
    };

    let result = list_paths(temp_dir.path().to_str().unwrap(), &options).unwrap();

    assert_eq!(result.len(), 3);
    assert!(result.iter().any(|p| p.ends_with("file1.txt")));
    assert!(result.iter().any(|p| p.ends_with("file2.rs")));
    assert!(result.iter().any(|p| p.ends_with("subdir")));
  }
  #[test]
  fn test_list_recursive_with_directories() {
    let temp_dir = create_test_directory();
    let options = ListOptions {
      recursive: true,
      include_directories: true,
      ..Default::default()
    };
    let result = list_paths(temp_dir.path().to_str().unwrap(), &options).unwrap();
    println!("Result: {:?}", result);
    assert_eq!(result.len(), 5);
    assert!(result.iter().any(|p| p.ends_with("file1.txt")));
    assert!(result.iter().any(|p| p.ends_with("file2.rs")));
    assert!(result.iter().any(|p| p.ends_with("subdir")));
    assert!(result.iter().any(|p| p.ends_with("subdir/file3.txt")));
    assert!(result.iter().any(|p| p.ends_with("subdir/file4.md")));
  }
  #[test]
  fn test_list_hidden_files() {
    let temp_dir = create_test_directory();
    let options = ListOptions {
      include_hidden: true,
      include_directories: false,
      ..Default::default()
    };
    let result = list_paths(temp_dir.path().to_str().unwrap(), &options).unwrap();
    assert!(!result.iter().any(|p| p.ends_with(".hidden_dir")));
    assert!(result.iter().any(|p| p.ends_with(".hidden_file")));
  }
  #[test]
  fn test_list_with_extension_filter() {
    let temp_dir = create_test_directory();
    let options = ListOptions {
      extensions: Some(vec!["txt".to_string()]),
      recursive: true,
      include_directories: false,
      ..Default::default()
    };
    let result = list_paths(temp_dir.path().to_str().unwrap(), &options).unwrap();
    assert_eq!(result.len(), 2);
    assert!(result.iter().any(|p| p.ends_with("file1.txt")));
    assert!(!result.iter().any(|p| p.ends_with("file2.rs")));
    assert!(!result.iter().any(|p| p.ends_with("subdir")));
  }
  #[test]
  fn test_list_paths_all_filters() {
    let temp_dir = create_test_directory();
    let options = ListOptions {
      recursive: true,
      include_hidden: true,
      include_directories: true,
      extensions: Some(vec!["txt".to_string()]),
    };
    let result = list_paths(temp_dir.path().to_str().unwrap(), &options).unwrap();
    assert!(result.iter().any(|p| p.ends_with("file1.txt")));
    assert!(!result.iter().any(|p| p.ends_with("file2.rs")));
    assert!(result.iter().any(|p| p.ends_with("subdir/file3.txt")));
    assert!(!result.iter().any(|p| p.ends_with("subdir/file4.md")));
    assert!(result.iter().any(|p| p.ends_with(".hidden_dir")));
  }
}
