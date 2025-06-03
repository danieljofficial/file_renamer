use std::{fs::read_dir, io::{Error, ErrorKind}, path::PathBuf};


pub fn list_directories(path: &str) -> Result<Vec<PathBuf>, Error> {
    let path = PathBuf::from(path);
    if !path.exists() {
        return Err(Error::new(
            ErrorKind::NotFound, format!("Path {} does not exist", path.display())
        ));
    };

    if !path.is_dir() {
        return Err(Error::new(ErrorKind::InvalidInput, format!("'{}' is not a directory", path.display())
        ));
    };

    let dir_entries = read_dir(&path)?;

    let mut directories = Vec::new();

    for entry in dir_entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            directories.push(path);
        }
    }

    Ok(directories)
}
#[cfg(test)]
pub mod tests {
    use super::*;
    use std::fs::{ create_dir, File};
    use tempfile::tempdir;

    #[test]
    fn successfully_list_directories() {
        let temp_dir = tempdir().unwrap();
        let dir1 = temp_dir.path().join("dir1");
        let dir2 = temp_dir.path().join("dir2");

        create_dir(&dir1).unwrap();
        create_dir(&dir2).unwrap();

        File::create(temp_dir.path().join("file.txt")).unwrap();

        let result = list_directories(temp_dir.path().to_str().unwrap()).unwrap();
        assert_eq!(result.len(), 2);
        assert!(result.contains(&dir1));
        assert!(result.contains(&dir2));
    }

    #[test]
    fn throws_correct_error_for_non_existent_directory() {
        let result = list_directories("bad/path");
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.kind(), ErrorKind::NotFound);
    }
    
    #[test]
    fn throws_correct_error_for_non_directories() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        File::create(&file_path).unwrap();

        let result = list_directories(file_path.to_str().unwrap());
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.kind(), ErrorKind::InvalidInput);
    }   
}