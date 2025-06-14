use crate::core::error::PathError;
use std::path::Path;

pub fn validate_pattern(pattern: &str) -> Result<(), PathError> {
  if pattern.contains("..") || pattern.contains("/") || pattern.contains("\\") {
    return Err(PathError::ValidationFailed(
      "Pattern cannot contian path navigation",
    ));
  }

  if pattern.contains(|c| matches!(c, '<' | '>' | ':' | '"' | '|' | '?' | '*' | '\0')) {
    return Err(PathError::ValidationFailed(
      "Pattern contains invalid characters",
    ));
  }

  let valid_placeholders = ["{name}", "{ext}", "{parent}", "{i}", "{date}"];

  if !valid_placeholders.iter().any(|ph| pattern.contains(ph)) {
    return Err(PathError::ValidationFailed(
      "Pattern must contain at least one placeholder: {name}, {ext}, {parent}, {i}, or {date}",
    ));
  }
  Ok(())
}

pub fn validate_rename(source: &Path, destination: &Path) -> Result<(), PathError> {
  if !source.exists() {
    return Err(PathError::NotFound(source.to_path_buf()));
  }

  if source.parent() != destination.parent() {
    return Err(PathError::ValidationFailed(
      "Cannot move files between directories",
    ));
  }

  if destination
    .to_string_lossy()
    .contains(|c| matches!(c, '<' | '>' | ':' | '"' | '|' | '?' | '*' | '\0'))
  {
    return Err(PathError::ValidationFailed(
      "Destination contains invalid characters",
    ));
  }
  Ok(())
}
