# TW3 - menu filelist updater
The nextgen version of the game introduced a filelist file the user must update for a menu XML file to appear in game. Since manually editing the files may be cumbersome, this utility can do it for you
while still making sure the DX11 file contains only the DX11 files.

# Downloading
 - The latest windows executable is available at this [direct download link](https://github.com/Aelto/tw3-menufilelist-updater/releases/latest/download/tw3-menufilelist-updater.exe)
 - Older versions are available in the [releases](https://github.com/Aelto/tw3-menufilelist-updater/releases) tab
 - Other operating systems will require to build the tool from the sources using the `rust` compiler

# Using it
## Binary
The binary can be placed in either the Witcher 3 game directory or in the `bin/config/r4game/user_config_matrix/pc` directory.

Running the binary will then cause it to list all the files in the filelists' directory and update them accordingly.

## ASI loader
> Technical explanation: the ASI laoder hooks itself to the game and runs all of the `.asi` libraries in the same directory as the witcher3.exe. ASI libraries are simple `.dll` files that were renamed to `.asi`.

If you wish to run automatically every time you launch the game then you use the [ASI loader](https://github.com/ThirteenAG/Ultimate-ASI-Loader) and the
ASI filelist updater library. The tool being pretty fast (benchmarked at at ~500ns per iteration)
you won't notice the difference and will always get updated filelists.

- Download the latest Ultimate ASI loader release: [direct download link](https://github.com/ThirteenAG/Ultimate-ASI-Loader/releases/latest/download/Ultimate-ASI-Loader.zip)
- Download the latest ASI menu filelist updater library: [direct download link](https://github.com/Aelto/tw3-menufilelist-updater/releases/latest/download/tw3_menufilelist_updater.asi)
- Drop both files `dinput8.dll` & `tw3_menufilelist_updater.asi` in
  - `The Witcher 3/bin/x64` if you use DX11
  - `The Witcher 3/bin/x64_dx12` if you use DX12
  - ... or in both folders!

Now confirm it is working by:
  - Emptying your filelists
  - Launching the game
  - Confirming the filelists are full again _(if you use the windows notepad you'll have to close the file and open it again)_

## Ignored files
Files that start with a `~` in their name are ignored.
