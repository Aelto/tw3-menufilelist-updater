use std::fmt::Display;
use std::path::PathBuf;

use crate::constants;

pub struct FileList(Vec<String>);

impl FileList {
  pub fn into_dx12_only_filelist(&self) -> String {
    let filtered_items: Vec<&String> = self
      .0
      .iter()
      .filter(|&filename| filename != constants::FILELIST_LAST_VANILLA_MENU_DX11)
      .filter(|&filename| !filename.starts_with(constants::FILE_IGNORE_PREFIX))
      .collect();

    FilteredFilelist(filtered_items).to_string()
  }

  pub fn into_dx11_only_filelist(&self) -> String {
    let filtered_items: Vec<&String> = self
      .0
      .iter()
      .filter(|&filename| filename != constants::FILELIST_LAST_VANILLA_MENU_DX12)
      .filter(|&filename| !filename.starts_with(constants::FILE_IGNORE_PREFIX))
      .collect();

    FilteredFilelist(filtered_items).to_string()
  }

  pub fn from_directory(dir: &PathBuf) -> std::io::Result<Self> {
    let entries = std::fs::read_dir(&dir)?;

    #[allow(unused_mut)]
    let mut entries: Vec<String> = entries
      .filter_map(|entry| entry.ok())
      .filter(|entry| {
        entry
          .path()
          .extension()
          .unwrap_or_default()
          .to_ascii_lowercase()
          == constants::FILE_EXTENSION
      })
      .filter_map(|entry| entry.file_name().into_string().ok())
      .map(|filename| filename.trim_matches(';').to_owned())
      .filter(|filename| {
        filename != constants::FILELIST_DX11 && filename != constants::FILELIST_DX12
      })
      .collect();

    // sort items during tests to ensure comparable results
    #[cfg(test)]
    entries.sort();

    Ok(entries.into())
  }
}

impl From<Vec<String>> for FileList {
  fn from(value: Vec<String>) -> Self {
    Self(value)
  }
}

impl From<Vec<&String>> for FileList {
  fn from(value: Vec<&String>) -> Self {
    Self(value.into_iter().map(|s| s.to_owned()).collect())
  }
}

impl AsRef<Vec<String>> for FileList {
  fn as_ref(&self) -> &Vec<String> {
    &self.0
  }
}

impl AsMut<Vec<String>> for FileList {
  fn as_mut(&mut self) -> &mut Vec<String> {
    &mut self.0
  }
}

pub struct FilteredFilelist<'a>(pub(crate) Vec<&'a String>);

impl<'a> Display for FilteredFilelist<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for filename in &self.0 {
      writeln!(f, "{filename};")?;
    }

    Ok(())
  }
}
