#![feature(drain_filter)]

pub mod constants;
mod filelist;
mod tests;

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

  let filelist = FileList::from_directory(&dir)?;

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
