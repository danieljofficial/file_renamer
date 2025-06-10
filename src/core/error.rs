use std::{io::Error, path::PathBuf};

#[derive(Debug)]
pub enum PathError {
    NotFound(PathBuf),
    NotADirectory(PathBuf),
    IoError(Error)    
}

impl From<Error> for PathError {
    fn from(err: Error) -> Self {
        PathError::IoError(err)
    }
}

impl std::fmt::Display for PathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathError::NotADirectory(path) => write!(f, "Path '{}' is not a directory!", path.display()),
            PathError::NotFound(path) => write!(f, "'{}' dooes not exist!", path.display()),
            PathError::IoError(error) => write!(f, "IO error: {}", error),
        }
    }
}

impl std::error::Error for  PathError {}