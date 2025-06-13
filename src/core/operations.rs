use std::{fs, path::{Path, PathBuf}};
// use crate::{core::{operations, rename::{self, RenameStrategy}}, RenameOptions};

use crate::{core::rename::{self, RenameStrategy}, RenameOptions};

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

fn list_paths_recursive(path: &Path, options: &ListOptions, result: &mut Vec<PathBuf>, is_recursive_call: bool) -> Result<(), PathError> {
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
                list_paths_recursive(&path, options, result, true)?;
            }

            if is_recursive_call || options.include_directories {
                result.push(path);
            }
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

pub fn rename_files(
    files: &[PathBuf],
    strategy: &dyn RenameStrategy,
    options: &RenameOptions
) -> Result<(), PathError> {
    for source in files {
        let destination = strategy.generate_name(source)?;

        rename::validate_rename(source, &destination)?;

        if options.dry_run {
            println!("Would rename {} to {}", source.display(), destination.display());
        continue;
        }

        perform_rename(source, &destination, options)?;
    };
    Ok(())
}

fn perform_rename(source: &Path, destination: &Path, options: &RenameOptions) -> Result<(), PathError> {
    unimplemented!()
}