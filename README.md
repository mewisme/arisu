# Arisu

A cute desktop pet animation powered by Bevy, featuring frame-based animations that can be controlled via global hotkeys.

## Features

- 🎬 Smooth frame-based animation (640 frames)
- 🖱️ Draggable window - move your pet anywhere on screen
- ⌨️ Global hotkey controls (no tray icon needed)
- 📌 Lock position to snap to taskbar
- 👁️ Hide/Show toggle
- ⏸️ Pause/Resume animation
- 🪟 Always on top, transparent window
- 💾 Embedded assets - all frames bundled in the executable

## Hotkeys

- **Alt+P**: Toggle Pause/Resume animation
- **Alt+H**: Toggle Hide/Visible window
- **Alt+L**: Toggle Lock/Unlock position (snaps to taskbar when locked)
- **Alt+Q**: Quit application

## Installation

### From Source

Requires Rust toolchain (1.70+):

```bash
git clone https://github.com/mewisme/arisu.git
cd arisu
cargo build --release
```

The compiled executable will be in `target/release/Arisu.exe`

## Usage

Simply run the executable. The animation will appear on your screen. Use the hotkeys to control it:

1. **Move**: Click and drag the animation window
2. **Lock to taskbar**: Press `Alt+L` to lock position and snap to taskbar
3. **Hide**: Press `Alt+H` to hide the window
4. **Pause**: Press `Alt+P` to pause the animation
5. **Quit**: Press `Alt+Q` to exit

## Requirements

- Windows 10/11 (tested)
- Display with taskbar support for snap-to-taskbar feature

## Project Structure

```
arisu/
├── assets/
│   └── frames/          # Animation frames (640 PNG files)
├── src/
│   ├── main.rs         # Entry point
│   ├── anim.rs         # Animation system
│   ├── drag.rs         # Window dragging
│   ├── hotkey.rs       # Global hotkey handler
│   ├── state.rs        # Application state
│   ├── system.rs       # System integration
│   └── taskbar.rs      # Taskbar detection (Windows)
└── res/
    └── icon.ico        # Application icon
```

## Technologies

- [Bevy](https://bevyengine.org/) - Game engine for rendering
- [bevy_embedded_assets](https://github.com/vleue/bevy_embedded_assets) - Asset embedding
- [global-hotkey](https://github.com/tauri-apps/global-hotkey) - Global hotkey support
- [Windows API](https://github.com/microsoft/windows-rs) - Windows integration

## License

MIT License - see [LICENSE](LICENSE) file for details

## Author

Mew <mauminh.nguyen@gmail.com>

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

