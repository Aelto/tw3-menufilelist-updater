#![feature(drain_filter)]
#![feature(test)]

mod constants;
mod filelist;
mod tests;

use filelist::attempts_updating;

fn main() {
  let result = attempts_updating("bin/config/r4game/user_config_matrix/pc".into())
    .and(attempts_updating("./".into()));

  if let Err(error) = result {
    println!("An error occured while updating the filelist: {error}");
  }
}
