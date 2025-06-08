use std::{fs::{self}, io::{Error, ErrorKind}, path::{Path, PathBuf}};

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

    let _ = list_paths_recursive(&path, options, &mut result);

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
    use tempfile::{ tempdir, TempDir};

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
        assert_eq!(result.len(), 5);
        assert!(result.iter().any(|p| p.ends_with("subdir")));
        assert!(result.iter().any(|p| p.ends_with("subdir/file3.txt")));
        assert!(result.iter().any(|p| p.ends_with("subdir/file4.md")));

    }

    #[test]
    fn test_list_paths_include_hidden() {
        let temp_dir = create_test_directory();
        let options = ListOptions {
            include_hidden: true,
            ..Default::default()
        };

        let result = list_paths(temp_dir.path().to_str().unwrap(), &options).unwrap();

        assert!(result.iter().any(|p| p.ends_with(".hidden_dir")));
        assert!(result.iter().any(|p| p.ends_with(".hidden_file")));
    }

    #[test]
    fn test_list_paths_filter_extensions() {
        let temp_dir = create_test_directory();
        let options = ListOptions {
            extensions: Some(vec!["txt".to_string(), "md".to_string()]),
            recursive: true,
            ..Default::default()
        };

        let result = list_paths(temp_dir.path().to_str().unwrap(), &options).unwrap();

        assert!(result.iter().any(|p| p.ends_with("file1.txt")));
        assert!(!result.iter().any(|p| p.ends_with("file2.rs")));
        assert!(result.iter().any(|p| p.ends_with("subdir/file3.txt")));
        assert!(result.iter().any(|p| p.ends_with("subdir/file4.md")));
    }

    #[test]
    fn test_list_paths_all_filters() {
        let temp_dir = create_test_directory();
        let options = ListOptions {
            recursive: true,
            include_hidden: true,
            extensions: Some(vec!["txt".to_string()])
        };

        let result = list_paths(temp_dir.path().to_str().unwrap(), &options).unwrap();

        assert!(result.iter().any(|p| p.ends_with("file1.txt")));
        assert!(!result.iter().any(|p| p.ends_with("file2.rs")));
        assert!(result.iter().any(|p| p.ends_with("subdir/file3.txt")));
        assert!(!result.iter().any(|p| p.ends_with("subdir/file4.md")));
        assert!(result.iter().any(|p| p.ends_with(".hidden_dir")));
    }

}