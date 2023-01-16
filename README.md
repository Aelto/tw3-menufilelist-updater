# TW3 - menu filelist updater
The nextgen version of the game introduced a filelist file the user must update for a menu XML file to appear in game. Since manually editing the files may be cumbersome, this utility can do it for you
while still making sure the DX11 file contains only the DX11 files.

# Downloading
 - The latest windows executable is available at this [direct download link](https://github.com/Aelto/tw3-menufilelist-updater/releases/latest/download/tw3-menufilelist-updater.exe)
 - Older versions are available in the [releases](https://github.com/Aelto/tw3-menufilelist-updater/releases) tab
 - Other operating systems will require to build the tool from the sources using the `rust` compiler

# Using it
The binary can be placed in either the Witcher 3 game directory or in the `bin/config/r4game/user_config_matrix/pc` directory.

Running the binary will then cause it to list all the files in the filelists' directory and update them accordingly.

## Ignored files
Files that start with a `~` in their name are ignored.
