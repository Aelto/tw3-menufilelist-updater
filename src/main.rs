#![feature(drain_filter)]

pub mod constants;
mod filelist;

use std::{error::Error, fs, path::PathBuf};

use filelist::FileList;

fn main() {
  let result = attempts_updating("bin/config/r4game/user_config_matrix/pc".into())
    .and(attempts_updating("./".into()));

  if let Err(error) = result {
    println!("An error occured while updating the filelist: {error}");
  }
}

fn attempts_updating(dir: PathBuf) -> Result<(), Box<dyn Error>> {
  println!("updating filelist.txt in {dir:?}");

  if !dir.exists() {
    println!(" - directory not found, skipping");

    return Ok(());
  }

  let entries = std::fs::read_dir(&dir)?;

  let entries: Vec<String> = entries
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
    .filter(|filename| filename != constants::FILELIST_DX11 && filename != constants::FILELIST_DX12)
    .collect();

  let filelist: FileList = entries.into();

  let dx11_output = dir.join(constants::FILELIST_DX11);
  if dx11_output.exists() {
    let dx11_content = dbg!(filelist.into_dx11_only_filelist());

    fs::write(dx11_output, dx11_content)?;
  }

  let dx12_output = dir.join(constants::FILELIST_DX12);
  if dx12_output.exists() {
    let dx12_content = dbg!(filelist.into_dx12_only_filelist());

    fs::write(dx12_output, dx12_content)?;
  }

  Ok(())
}
