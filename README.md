# Spray

A lightweight desktop pet application with smooth GIF animations. Features an embedded default animation that works immediately, plus full customization support for your own GIF files or PNG frame sequences.

## Features

- 🎬 **Smooth animations** - Supports GIF and PNG frame sequences
- 💾 **Embedded default** - Works out of the box with embedded `evernight.gif` animation
- 🎨 **Fully customizable** - Use your own GIF files or PNG frames
- 🎯 **Scale control** - Adjust animation size with `scale_percent` config (default: 40%)
- 🔄 **Hot reload** - Config changes apply instantly without restart
- 🖱️ **Draggable window** - Click and drag to move anywhere on screen
- 📌 **Auto snap to taskbar** - Automatically positions above taskbar on startup
- 🪟 **Always on top** - Transparent window that stays visible

## Usage

Simply run the executable. The app will automatically load the embedded `evernight.gif` animation and appear above your taskbar. Click and drag to move it to any position on your screen.

### Configuration

On first run, `spray.config.json` will be auto-created with these defaults:

```json
{
  "fps": 12,
  "auto_startup": false,
  "frame_digits": 4,
  "frame_width": 200.0,
  "frame_height": 250.0,
  "window_title": "Spray",
  "frame_folder": "frames",
  "gif_path": "evernight.gif",
  "mode": "auto",
  "scale_percent": 40.0
}
```

### Customization

You can customize the animation in several ways:

1. **Custom GIF**: Place a GIF file in `assets/` folder (e.g., `assets/my_animation.gif` and set `gif_path` in config)
2. **Custom PNG Frames**: Create `assets/frames/` folder with PNG frames (`frame_0001.png`, `frame_0002.png`, etc.)
3. **Adjust FPS**: Change `fps` value in config (default: 12)
4. **Adjust Scale**: Change `scale_percent` value (default: 40.0 = 40% of original size)
5. **Window Size**: Adjust `frame_width` and `frame_height` (default: 200x250)

**Animation Loading Priority:**
1. External GIF from `assets/{gif_path}` (if exists)
2. External PNG frames from `assets/{frame_folder}/` (if exists)
3. Embedded GIF (`evernight.gif`) - **default, always available**
4. Embedded PNG frames (fallback)

All config changes apply instantly via hot reload - no restart needed!

See [CUSTOM_FRAMES.md](CUSTOM_FRAMES.md) for detailed instructions.

## Installation

### From Source

Requires Rust toolchain (1.70+):

```bash
git clone https://github.com/mewisme/spray.git
cd spray
cargo build --release
```

The compiled executable will be in `target/release/Spray.exe`

### From Release

Download the latest `Spray.exe` from [Releases](https://github.com/mewisme/spray/releases) and run it.

## Requirements

- **OS**: Windows 10/11 (64-bit)
- **GPU**: DirectX 11 compatible
- **RAM**: 100MB minimum (200MB recommended)
- **Disk**: 100MB for executable
- **Display**: Taskbar support for auto-snap feature (optional)

No additional runtime or dependencies needed - everything is bundled in the executable!

## Project Structure

```
spray/
├── assets/
│   ├── evernight.gif         # Default embedded GIF animation
│   ├── frames/               # Optional: Custom PNG frames
│   └── icon.ico              # Application icon
└── src/
    ├── main.rs               # Entry point
    ├── animation/            # Animation system
    │   ├── mod.rs
    │   └── anim.rs          # Frame animation logic
    ├── config/               # Configuration management
    │   ├── mod.rs
    │   ├── config.rs        # Config load/save
    │   ├── config_watcher.rs # Hot reload watcher
    │   └── config_applier.rs # Apply config changes
    ├── window/               # Window management
    │   ├── mod.rs
    │   ├── drag.rs          # Window dragging
    │   └── state.rs         # Application state
    └── platform/             # Platform-specific (Windows)
        ├── mod.rs
        ├── system.rs        # System integration
        └── taskbar.rs       # Taskbar detection
```

## Technologies

- [Bevy](https://bevyengine.org/) - Game engine for rendering
- [bevy_embedded_assets](https://github.com/vleue/bevy_embedded_assets) - Asset embedding
- [Windows API](https://github.com/microsoft/windows-rs) - Windows integration
- [image](https://github.com/image-rs/image) - GIF decoding and image processing

## License

MIT License - see [LICENSE](LICENSE) file for details

## Author

Mew <mauminh.nguyen@gmail.com>

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.
