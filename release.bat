
cargo build --release

rmdir /s /q release
mkdir release
move target\release\tw3-menufilelist-updater.exe release\tw3-menufilelist-updater.exe
move target\release\tw3_menufilelist_updater.dll release\tw3_menufilelist_updater.asi