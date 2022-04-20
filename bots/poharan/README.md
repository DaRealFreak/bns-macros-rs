# Poharan Bot
[![build - poharan](https://github.com/DaRealFreak/bns-macros-rs/actions/workflows/build_poharan.yml/badge.svg)](https://github.com/DaRealFreak/bns-macros-rs/actions/workflows/build_poharan.yml)  
Bot to farm the dungeon "Chaos Supply Chain" on your preferred stage automatically. Use at your own risk, this can get you banned if NCSoft suddenly decides that they care about their game again.

## Installation
Before setting up the bot you'll need [Cheat Engine](https://www.cheatengine.org/) for the Fly Hack and Lobby Speed Hack parts. Every other cheat is applied directly in the memory.  
Create a folder `configuration` and run the `bns-poharan.exe` in your preferred directory. This will create a basic `poharan.ini` file which is used to change keys and color checks to match your key bindings and ClientConfiguration.xml.

## Cheat Tables
For the bot to fully work, you'll the clipper for the warlock portal. This is **NOT** included in this project and probably won't ever be included, since it was not made by myself.

## Configuration
For Keybindings you should refer to [Virtual-Key Codes](https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes).
For Positions and Colors you can use external tools like AutoHotkey scripts or Snipping Tools.
The bns-macros project of this repository also outputs the coordinates/hex color of the current mouse position which can be used to set everything up.

## Usage
After changing all the settings in the `poharan.ini` you can simply run the `bns-poharan.exe` and tab into the game on the client of your warlock.  
If you want to stop the bot simply tab out of the game or close the console window.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
