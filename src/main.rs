// hide console window on release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(test)]

mod constants;
pub mod error;
mod filelist;
mod tests;

#[cfg(feature = "summary")]
mod summary;

use filelist::attempts_updating;

fn main() {
  let mut errors = Vec::new();

  let a = error::handle(
    &mut errors,
    attempts_updating(constants::MENU_DIRECTORY.into()),
  );
  let b = error::handle(&mut errors, attempts_updating("./".into()));

  if !errors.is_empty() && a.is_none() && b.is_none() {
    errors.push(error::FilelistError::from(
      "\nUnsuccessful attempts at updating the filelists, please ensure the tool is placed in your Witcher 3 game's root directory",
    ));

    summary::display_summary(Default::default(), errors);
  }
}
