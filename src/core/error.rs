use std::{io::Error, path::PathBuf};

#[derive(Debug)]
pub enum PathError {
  NotFound(PathBuf),
  NotADirectory(PathBuf),
  IoError(Error),
  RenameFailed {
    source: PathBuf,
    destination: PathBuf,
    cause: Error,
  },
  InvalidFileRenamePattern,
  DestinationExists(PathBuf),
  ValidationFailed(&'static str),
}

impl From<Error> for PathError {
  fn from(err: Error) -> Self {
    PathError::IoError(err)
  }
}

impl std::fmt::Display for PathError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      PathError::NotADirectory(path) => {
        write!(f, "Path '{}' is not a directory!", path.display())
      }
      PathError::NotFound(path) => write!(f, "'{}' dooes not exist!", path.display()),
      PathError::IoError(error) => write!(f, "IO error: {}", error),
      PathError::RenameFailed {
        source,
        destination,
        cause,
      } => write!(
        f,
        "Failed to rename '{}' to '{}': {}",
        source.display(),
        destination.display(),
        cause
      ),
      PathError::InvalidFileRenamePattern => write!(
        f,
        "Invalid rename pattern. Patterns must contain valid placeholders like {{name}}, {{ext}}, etc."
      ),
      PathError::DestinationExists(path) => write!(
        f,
        "Destination path '{}' already exists. Use overwrite option or choose a different name.",
        path.display()
      ),
      PathError::ValidationFailed(msg) => write!(f, "Validation failed: {}", msg),
    }
  }
}

impl std::error::Error for PathError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      PathError::IoError(e) | PathError::RenameFailed { cause: e, .. } => Some(e),
      _ => None,
    }
  }
}
