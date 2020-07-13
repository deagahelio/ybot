# ybot

A Geometry Dash replay bot for Linux.

# Usage

Run `sudo -E cargo run --release`, or compile it normally and run the binary as root.

- F2 = load replay
- F3 = save replay
- F4 = recording mode, pause/unpause recording
- F5 = playback mode
- F6 = clear replay
- F8 = update level addresses (PRESS EVERY TIME YOU CHANGE LEVELS, or else the program will not work properly)
- F9 = toggle smart recording mode (for recording replays in practice mode, UNTOGGLE WHEN GOING OUT OF PRACTICE or it will clear the entire replay)

The bot detects inputs on space bar only, and sends arrow key inputs.

Should work on X and Wayland, tested on Arch Linux with i3.

# Todo

- [ ] Configurable keybinds
- [ ] Automatically update level addresses
- [ ] Better user interface
- [ ] Minimize desync/lag