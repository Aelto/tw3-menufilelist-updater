use std::fmt::Display;

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

pub struct FilteredFilelist<'a>(Vec<&'a String>);

impl<'a> Display for FilteredFilelist<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for filename in &self.0 {
      writeln!(f, "{filename};")?;
    }

    Ok(())
  }
}
