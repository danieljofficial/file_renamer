use std::path::{Path, PathBuf};

use regex::Regex;

use crate::core::{PathError, rename::validator};

pub trait RenameStrategy {
  fn generate_name(&mut self, original: &Path) -> Result<PathBuf, PathError>;
}

#[derive(Debug)]
pub struct PatternRename {
  pattern: String,
  _compiled: Regex,
  current_index: usize,
}

impl PatternRename {
  pub fn new(pattern: String) -> Result<Self, PathError> {
    validator::validate_pattern(&pattern)?;

    // let regex_pattern = pattern
    //   .replace("{name}", "(?P<name>[^/]+?)")
    //   .replace("{ext}", "(?P<ext>\\.[^.]+)?")
    //   .replace("{parent}", "(?P<parent>[^/]+)")
    //   .replace("{i}", "(?P<i>\\d+)")
    //   .replace("{date}", "(?P<date>\\d{4}-\\d{2}-\\d{2})");

    // let compiled = Regex::new(&format!("^{}$", regex_pattern))
    //   .map_err(|_| PathError::InvalidFileRenamePattern)?;

    let compiled = Regex::new(&build_regex_pattern(&pattern))
      .map_err(|_| PathError::InvalidFileRenamePattern)?;

    Ok(Self {
      pattern,
      _compiled: compiled,
      current_index: 0,
    })
  }

  fn resolve_placeholders(&self, original: &Path) -> Result<String, PathError> {
    let file_stem = original
      .file_stem()
      .and_then(|s| s.to_str())
      .ok_or_else(|| PathError::ValidationFailed("Invalid filename"))?;

    let extension = original
      .extension()
      .map(|ext| ext.to_string_lossy())
      .unwrap_or_default();

    let parent = original
      .parent()
      .and_then(|p| p.file_name())
      .and_then(|s| s.to_str())
      .unwrap_or("");

    let now = chrono::Local::now().format("%Y-%m-%d").to_string();

    let mut result = self.pattern.clone();
    // let dotted_ext = format!(".{}", extension);

    result = result.replace("{name}", file_stem);
    result = result.replace("{parent}", parent);
    result = result.replace("{ext}", &extension);
    result = result.replace("{i}", &self.current_index.to_string());
    result = result.replace("{date}", &now);

    Ok(result)
  }
}

impl RenameStrategy for PatternRename {
  fn generate_name(&mut self, original: &Path) -> Result<PathBuf, PathError> {
    self.current_index += 1;
    let new_name = self.resolve_placeholders(original)?;
    let parent_dir = original.parent().unwrap_or_else(|| Path::new("."));

    Ok(parent_dir.join(new_name))
  }
}

fn build_regex_pattern(pattern: &str) -> String {
  pattern
    .replace("{name}", "(?P<name>[^/]+?)")
    // .replace("{ext}", "(?P<ext>\\.[^.]+)?")
    .replace("{ext}", "(?P<ext>\\.+)?")
    .replace("{parent}", "(?P<parent>[^/]+)")
    .replace("{i}", "(?P<i>\\d+)")
    .replace("{date}", "(?P<date>\\d{4}-\\d{2}-\\d{2})")
}
