use std::ops::Deref;
use std::path::PathBuf;

use crate::constants;
use crate::error::FilelistError;
use crate::summary::Summary;

pub fn attempts_updating(dir: PathBuf) -> Result<(), FilelistError> {
  if !dir.exists() {
    return Err(FilelistError::FilelistDirectoryNotFound(dir));
  }

  let dx11_output = dir.join(constants::FILELIST_DX11);
  let dx12_output = dir.join(constants::FILELIST_DX12);
  let dx11_exists = dx11_output.exists();
  let dx12_exists = dx12_output.exists();

  if !dx11_exists && !dx12_exists {
    return Err(FilelistError::FilelistDirectoryNotFound(dir));
  }

  // mutable "stores" that are filled as we construct the filelists,
  // - errors may accumulate as we progress due to various reasons, incorrect
  //   path, file locks etc...
  // - a summary is built once, only where the current value is None even if
  //   both dx11 & dx12 filelists are updated
  let mut errors = Vec::new();
  let mut summary: Option<Summary> = None;

  let Some(filelist) = crate::error::handle(&mut errors, FileList::from_directory(&dir)) else {
    return Ok(());
  };

  if dx11_exists {
    let dx11_content = filelist.to_dx11_only_filelist();

    let old_content = crate::error::handle(&mut errors, std::fs::read_to_string(&dx11_output));

    crate::error::handle(&mut errors, std::fs::write(dx11_output, &dx11_content));

    if let Some(old_content) = old_content {
      summary = Some(Summary {
        new_content: dx11_content,
        old_content,
      });
    }
  }

  if dx12_exists {
    let dx12_content = filelist.to_dx12_only_filelist();

    // the old content is only fetched if a summary is needed
    let old_content = match summary.is_none() {
      true => crate::error::handle(&mut errors, std::fs::read_to_string(&dx12_output)),
      false => None,
    };

    crate::error::handle(&mut errors, std::fs::write(dx12_output, &dx12_content));

    if let Some(old_content) = old_content {
      summary = Some(Summary {
        new_content: dx12_content,
        old_content,
      });
    }
  }

  if let Some(summary) = summary {
    crate::summary::display_summary(summary, errors);
  }

  Ok(())
}

pub struct FileList {
  pub menus: Vec<String>,
}

impl FileList {
  #[cfg(test)]
  pub fn new(mut menus: Vec<String>) -> Self {
    // sort items during tests to ensure comparable results
    menus.sort();

    Self { menus }
  }

  #[cfg(not(test))]
  pub fn new(menus: Vec<String>) -> Self {
    Self { menus }
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

    Ok(Self::new(entries))
  }

  pub fn raw_entries(&self) -> impl Iterator<Item = &str> {
    self.menus.iter().map(|s| s.deref())
  }

  fn entries(&self) -> impl Iterator<Item = &str> {
    self
      .raw_entries()
      .filter(|filename| !filename.starts_with(constants::FILE_IGNORE_PREFIX))
  }

  pub fn dx11_entries(&self) -> impl Iterator<Item = &str> {
    self
      .entries()
      .filter(|&filename| filename != constants::FILELIST_LAST_VANILLA_MENU_DX12)
  }

  pub fn dx12_entries(&self) -> impl Iterator<Item = &str> {
    self
      .entries()
      .filter(|&filename| filename != constants::FILELIST_LAST_VANILLA_MENU_DX11)
  }

  pub fn to_dx12_only_filelist(&self) -> String {
    let mut out = String::new();

    for entry in self.dx12_entries() {
      out.push_str(entry);
      out.push(';');
      out.push('\n');
    }

    out
  }

  pub fn to_dx11_only_filelist(&self) -> String {
    let mut out = String::new();

    for entry in self.dx11_entries() {
      out.push_str(entry);
      out.push(';');
      out.push('\n');
    }

    out
  }
}

impl AsRef<Vec<String>> for FileList {
  fn as_ref(&self) -> &Vec<String> {
    &self.menus
  }
}

impl AsMut<Vec<String>> for FileList {
  fn as_mut(&mut self) -> &mut Vec<String> {
    &mut self.menus
  }
}
