pub mod constants;
pub mod filelist;

use std::path::Path;

use filelist::attempts_updating;

#[no_mangle] // needs to precede every function that is called from c
extern "stdcall" fn DllMain(a: *const u8, b: u32, c: *const u8) -> u32 {
  // the program is launched  multiple times, once for the launcher and once
  // for the actual game.
  let _is_launcher = b == 0;
  let is_game = b == 1;

  if is_game {
    let game_root_directory = Path::new("../../");
    let menu_directory = game_root_directory.join("bin/config/r4game/user_config_matrix/pc");
    let result = attempts_updating(menu_directory);

    if let Err(error) = result {
      println!("An error occured while updating the filelist: {error}");
    }
  }

  0
}
