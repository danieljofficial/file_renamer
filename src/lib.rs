use std::{fs::{self, read_dir}, io::{Error, ErrorKind}, path::{Path, PathBuf}};

#[derive(Debug, Default)]
pub struct ListOptions {
   pub recursive: bool,
   pub include_hidden: bool,
   pub extensions: Option<Vec<String>>,

}


pub fn list_paths(path: &str, options: &ListOptions) -> Result<Vec<PathBuf>, Error> {
    let path = PathBuf::from(path);
    if !path.exists() {
        return Err(Error::new(
            ErrorKind::NotFound, 
            format!("Path {} does not exist", path.display())
        ));
    };

    if !path.is_dir() {
        return Err(Error::new(
            ErrorKind::InvalidInput, 
            format!("'{}' is not a directory", path.display())
        ));
    };

    let mut result = Vec::new();

    list_paths_recursive(&path, options, &mut result);

    Ok(result)
}

fn list_paths_recursive(path: &Path, options: &ListOptions, result: &mut Vec<PathBuf>) -> Result<(), Error> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        let is_hidden = path.file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.starts_with("."))
            .unwrap_or(false);

        if is_hidden && !options.include_hidden {
            continue;
        };

        if path.is_dir() {
            if options.recursive {
                list_paths_recursive(&path, options, result)?;
            }
            result.push(path);
        } else {
            let should_include = match &options.extensions {
                Some(exts) => path.extension()
                    .and_then(|e| e.to_str())
                    .map(|e| exts.iter().any(|x| x.eq_ignore_ascii_case(e)))
                    .unwrap_or(false),

                None => true,
            };

            if should_include {
                result.push(path)
            }
        }

    }
    Ok(())
}
#[cfg(test)]
pub mod tests {
    use super::*;
    use std::fs::{ self, File};
    use tempfile::{tempdir, TempDir};

    fn create_test_directory() -> TempDir{
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

    #[test]
    fn test_list_paths_non_recursive() {
        let temp_dir = create_test_directory();
        let options = ListOptions::default();

        let result = list_paths(temp_dir.path().to_str().unwrap(), &options).unwrap();

        assert_eq!(result.len(), 3);
        assert!(result.iter().any(|p| p.ends_with("subdir")));
        assert!(result.iter().any(|p| p.ends_with("file1.txt")));
        assert!(result.iter().any(|p| p.ends_with("file2.rs")));

    }

    #[test]
    fn test_list_paths_recursive() {
        let temp_dir = create_test_directory();  
        let options = ListOptions {
            recursive: true, 
            ..Default::default()
        };

        let result = list_paths(temp_dir.path().to_str().unwrap(), &options).unwrap(); 

        println!("Result: {:?}", result);
        assert_eq!(result.len(), 6);
        assert!(result.iter().any(|p| p.ends_with("subdir")));
        assert!(result.iter().any(|p| p.ends_with("subdir/file3.txt")));
        assert!(result.iter().any(|p| p.ends_with("subdir/file4.md")));

    }

    // #[test]
    // fn successfully_list_directories() {
    //     let temp_dir = tempdir().unwrap();
    //     let dir1 = temp_dir.path().join("dir1");
    //     let dir2 = temp_dir.path().join("dir2");

    //     create_dir(&dir1).unwrap();
    //     create_dir(&dir2).unwrap();

    //     File::create(temp_dir.path().join("file.txt")).unwrap();

    //     let result = list_directories(temp_dir.path().to_str().unwrap()).unwrap();
    //     assert_eq!(result.len(), 2);
    //     assert!(result.contains(&dir1));
    //     assert!(result.contains(&dir2));
    // }

    // #[test]
    // fn throws_correct_error_for_non_existent_directory() {
    //     let result = list_directories("bad/path");
    //     assert!(result.is_err());
    //     let error = result.unwrap_err();
    //     assert_eq!(error.kind(), ErrorKind::NotFound);
    // }
    
    // #[test]
    // fn throws_correct_error_for_non_directories() {
    //     let temp_dir = tempdir().unwrap();
    //     let file_path = temp_dir.path().join("test.txt");

    //     File::create(&file_path).unwrap();

    //     let result = list_directories(file_path.to_str().unwrap());
    //     assert!(result.is_err());
    //     let error = result.unwrap_err();
    //     assert_eq!(error.kind(), ErrorKind::InvalidInput);
    // }   
}