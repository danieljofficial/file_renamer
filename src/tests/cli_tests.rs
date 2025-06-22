#[cfg(test)]
pub mod tests {
  use assert_cmd::Command;

  use crate::{ListOptions, core::list_paths, tests::helpers::create_test_directory};
  use predicates::prelude::*;

  #[test]
  fn test_cli_basic_rename() {
    let temp_dir = create_test_directory();
    let mut cmd = Command::cargo_bin("ichimonji").expect("Failed to find binary");

    let assert = cmd
      .arg("rename")
      .arg("-d")
      .arg(temp_dir.path())
      .arg("-r")
      .arg("{name}_backup.{ext}")
      .arg("-R")
      .assert();

    assert
      .success()
      .stdout(predicate::str::contains("Successfully renamed"));

    // println!("paths: {:#?}", temp_dir.path());
    let condition1 = temp_dir.path().join("file1_backup.txt").exists();
    let condition2 = temp_dir.path().join("subdir/file3_backup.txt").exists();
    println!("condition {}", &condition2);
    assert!(condition1);
    assert!(condition2);
  }
  #[test]
  fn test_cli_dry_run() {
    let temp_dir = create_test_directory();
    let mut cmd = Command::cargo_bin("ichimonji").unwrap();

    let assert = cmd
      .arg("rename")
      .arg("-d")
      .arg(temp_dir.path())
      .arg("-r")
      .arg("{name}_v{i}.{ext}")
      .arg("--dry-run")
      .assert();

    assert
      .success()
      .stdout(predicate::str::contains("DRY RUN"))
      .stdout(predicate::str::contains("file1_v2.txt"));

    assert!(temp_dir.path().join("file1.txt").exists());
  }

  #[test]
  fn test_cli_recursive_pattern() {
    let temp_dir = create_test_directory();
    let mut cmd = Command::cargo_bin("ichimonji").unwrap();

    let assert = cmd
      .arg("rename")
      .arg("-d")
      .arg(temp_dir.path())
      .arg("-R")
      .arg("-r")
      .arg("{parent}_{name}_{date}.{ext}")
      .assert();

    assert
      .success()
      .stdout(predicate::str::contains("Successfully renamed"));

    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let expected = format!("subdir_file3_{}.txt", date);

    // let options = ListOptions {
    //   include_directories: true,
    //   recursive: true,
    //   include_hidden: true,
    //   ..Default::default()
    // };
    // let result = list_paths(temp_dir.path().to_str().unwrap(), &options).unwrap();
    // println!("results: {:#?}", result);
    // assert!(temp_dir.path().join("file1.txt").exists());
    assert!(temp_dir.path().join("subdir").join(expected).exists());

    // When testing recursive rename keep in mind that the temp_file parent dir
    // is a hidden dir. So you dont lose your mind over where the other non-nested renamed temporary files went.
  }

  #[test]
  fn test_cli_extension_filter() {
    let temp_dir = create_test_directory();
    let mut cmd = Command::cargo_bin("ichimonji").unwrap();

    let assert = cmd
      .arg("rename")
      .arg("-d")
      .arg(temp_dir.path())
      .arg("-p")
      .arg("*.txt")
      .arg("-R")
      .arg("-r")
      .arg("text_{name}.{ext}")
      .assert();

    assert
      .success()
      .stdout(predicate::str::contains("Successfully renamed"));

    let options = ListOptions {
      include_directories: true,
      recursive: true,
      include_hidden: true,
      ..Default::default()
    };
    let result = list_paths(temp_dir.path().to_str().unwrap(), &options).unwrap();
    println!("results: {:#?}", result);

    assert!(temp_dir.path().join("text_file1.txt").exists());
    assert!(temp_dir.path().join("file2.rs").exists());
    assert!(
      temp_dir
        .path()
        .join("subdir")
        .join("text_file3.txt")
        .exists()
    );
  }

  #[test]
  fn test_cli_invalid_pattern() {
    // let temp_dir = create_test_directory();
    //  Provide a better error message
    let mut cmd = Command::cargo_bin("ichimonji").unwrap();
    cmd
      .arg("rename")
      .arg("-r")
      .arg("invalid{pattern")
      .assert()
      .failure()
      .stderr(predicate::str::contains("Validation failed"));
  }
}
