use std::{
  fs,
  path::{Path, PathBuf},
};
// use crate::{core::{operations, rename::{self, RenameStrategy}}, RenameOptions};

use crate::{
  ConflictResolution, RenameOptions,
  core::rename::{self, RenameStrategy, validator},
};

use super::error::PathError;
use super::options::ListOptions;

pub fn list_paths(path: &str, options: &ListOptions) -> Result<Vec<PathBuf>, PathError> {
  let path = PathBuf::from(path);
  if !path.exists() {
    return Err(PathError::NotFound(path));
  };

  if !path.is_dir() {
    return Err(PathError::NotADirectory(path));
  };

  let mut result = Vec::new();

  let _ = list_paths_recursive(&path, options, &mut result, false)?;

  Ok(result)
}

fn list_paths_recursive(
  path: &Path,
  options: &ListOptions,
  result: &mut Vec<PathBuf>,
  is_recursive_call: bool,
) -> Result<(), PathError> {
  for entry in fs::read_dir(path)? {
    let entry = entry?;
    let path = entry.path();

    let is_hidden = path
      .file_name()
      .and_then(|n| n.to_str())
      .map(|s| s.starts_with("."))
      .unwrap_or(false);

    if is_hidden && !options.include_hidden {
      continue;
    };

    if path.is_dir() {
      if options.recursive {
        list_paths_recursive(&path, options, result, true)?;
      }

      if is_recursive_call || options.include_directories {
        result.push(path);
      }
    } else {
      let should_include = match &options.extensions {
        Some(exts) => path
          .extension()
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

pub fn rename_files(
  files: &[PathBuf],
  strategy: &mut dyn RenameStrategy,
  options: &RenameOptions,
) -> Result<(), PathError> {
  for (i, source) in files.iter().enumerate() {
    let destination = strategy.generate_name(source)?;

    // rename::validate_rename(source, &destination)?;
    validator::validate_rename(source, &destination)?;

    if options.dry_run {
      println!(
        "[Dry run] Would rename '{}' to '{}'",
        source.display(),
        destination.display()
      );
      continue;
    }

    perform_rename(source, &destination, options, i)?;
  }
  Ok(())
}

fn perform_rename(
  source: &Path,
  destination: &Path,
  options: &RenameOptions,
  index: usize,
) -> Result<(), PathError> {
  if destination.exists() {
    match options.conflict_resolution {
      ConflictResolution::Skip => {
        println!("Skipping '{}' - destination exists", source.display());
        return Ok(());
      }
      ConflictResolution::Overwrite if options.overwrite => {
        fs::remove_file(destination).map_err(|e| PathError::RenameFailed {
          source: source.to_path_buf(),
          destinaiton: destination.to_path_buf(),
          cause: e,
        })?;
      }
      ConflictResolution::Numbered => {
        let new_destination = find_available_name(destination)?;
        return perform_rename(source, &new_destination, options, index);
      }
      _ => return Err(PathError::DestinationExists(destination.to_path_buf())),
    }
  };

  fs::rename(source, destination).map_err(|e| PathError::RenameFailed {
    source: source.to_path_buf(),
    destinaiton: destination.to_path_buf(),
    cause: e,
  })
}

fn find_available_name(original: &Path) -> Result<PathBuf, PathError> {
  let parent = original.parent().unwrap_or_else(|| Path::new("."));
  let stem = original.file_stem().and_then(|s| s.to_str()).unwrap_or("");
  let extension = original
    .extension()
    .and_then(|ext| ext.to_str())
    .map(|ext| format!(".{}", ext))
    .unwrap_or_default();

  for i in 1..1000 {
    let new_name = format!("{}_{}{}", stem, i, extension);
    let new_path = parent.join(new_name);

    if !new_path.exists() {
      return Ok(new_path);
    }
  }

  Err(PathError::ValidationFailed(
    "Could not find available filename after 1000 attempts",
  ))
}
