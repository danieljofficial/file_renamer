#[derive(Debug, Default)]
pub struct ListOptions {
  pub recursive: bool,
  pub include_hidden: bool,
  pub include_directories: bool,
  pub extensions: Option<Vec<String>>,
}
#[derive(Debug)]
pub struct RenameOptions {
  pub dry_run: bool,
  pub overwrite: bool,
  pub conflict_resolution: ConflictResolution,
}

#[derive(Debug)]
pub enum ConflictResolution {
  Skip,
  Overwrite,
  Numbered,
}

impl Default for RenameOptions {
  fn default() -> Self {
    Self {
      dry_run: false,
      overwrite: false,
      conflict_resolution: ConflictResolution::Numbered,
    }
  }
}
