use std::{io::Error, path::PathBuf};

#[derive(Debug)]
pub enum PathError {
    NotFound(PathBuf),
    NotADirectory(PathBuf),
    IoError(Error),
    RenameFailed {
        source: PathBuf,
        destinaiton: PathBuf,
        cause: Error,
    },
    InvalidFileRenamePattern,
    DestinationExists(PathBuf),
    ValidationFailed(&'static str)   
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
            PathError::RenameFailed { source, destinaiton, cause } => todo!(),
            PathError::InvalidFileRenamePattern => todo!(),
            PathError::DestinationExists(path_buf) => todo!(),
            PathError::ValidationFailed(_) => todo!(),
                    }
    }
}

impl std::error::Error for  PathError {}