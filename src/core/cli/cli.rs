use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::{ListOptions, PatternRename, RenameOptions, core::list_paths, rename_files};

#[derive(Parser)]
#[command(name = "rename")]
#[command(about = "File renamer", version, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Rename {
    /// Root dir
    #[arg(short = 'd', long, default_value = ".")]
    path: PathBuf,

    /// file name pattern
    #[arg(short = 'p', long)]
    // #[arg(long)]
    pattern: Option<String>,

    /// Rename pattern
    #[arg(short = 'r', long)]
    rename: String,

    /// Include hidden options
    #[arg(long)]
    hidden: bool,

    /// Recursive search
    #[arg(short = 'R', long)]
    recursive: bool,

    /// Dry run (show changes without applying)
    #[arg(long)]
    dry_run: bool,
  },
}

pub fn run() -> anyhow::Result<()> {
  let cli = Cli::parse();

  match cli.command {
    Commands::Rename {
      path,
      pattern,
      rename,
      hidden,
      recursive,
      dry_run,
    } => {
      let list_options = ListOptions {
        recursive,
        include_hidden: hidden,
        extensions: pattern.map(|p| vec![p.replace("*.", "")]),
        ..Default::default()
      };

      let rename_options = RenameOptions {
        dry_run,
        ..Default::default()
      };

      let mut renamer = PatternRename::new(rename)?;

      let files = list_paths(path.to_str().unwrap(), &list_options)?;

      if dry_run {
        println!("DRY RUN: would rename {} files", files.len());
      }

      rename_files(&files, &mut renamer, &rename_options)?;

      if dry_run {
        println!("No changes made (dry run)")
      } else {
        println!("Successfully renamed {} files", files.len());
      }
    }
  }
  Ok(())
}
