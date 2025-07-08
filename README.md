# EDFI

A lightweight, vim-like modal terminal text editor written in Rust using Ratatui.

## Features

### Current Features

- **Modal Editing**: Vi-inspired dual-mode editing system
- **File Operations**: Create, open, and save files
- **Smart Cursor Movement**: Navigate efficiently through text
- **Command Line Integration**: Open files directly from terminal
- **Auto-scrolling**: Smooth scrolling when cursor moves beyond visible area
- **Line Management**: Automatic line creation and deletion
- **Backspace Handling**: Intelligent line merging and deletion
- **Horizontal Scroll**: Not implemented yet.

### Modes

#### Normal Mode
- `i` - Enter editing mode
- `s` - Save current file
- `o` - Open/reload file
- `q` - Quit editor
- Arrow keys - Move cursor
- `Home` - Move to beginning of line
- `End` - Move to end of line

#### Editing Mode
- `ESC` - Return to normal mode
- `Enter` - Create new line
- `Backspace` - Delete character or merge lines
- Arrow keys - Move cursor
- `Home` - Move to beginning of line
- `End` - Move to end of line
- Any printable character - Insert at cursor position

## Installation

Make sure you have Rust installed, then:

```bash
git clone https://github.com/0l3d/edfi.git
cd edfi
cargo build --release
```

## Usage

### Create a new file
```bash
./target/release/edfi
```

### Open an existing file
```bash
./target/release/edfi filename.txt
```

### Open a non-existent file (creates new file)
```bash
./target/release/edfi newfile.txt
```

## Images

### Configuration
![image](https://github.com/user-attachments/assets/045df880-4f64-4c44-94cd-b9ab68a0c39e)

*[ffetch](https://github.com/0l3d/ffetch) configuration with EDFI*

### Rust Code (coming soon...)

## Dependencies

- `ratatui` - Terminal user interface library
- `color_eyre` - Error handling and reporting
- `crossterm` - Cross-platform terminal manipulation

## Planned Features

- **Search Functionality**: Find and replace text within files
- **Lua Script Support**: Extend editor functionality with Lua scripting
- **Rust Syntax Highlighting**: Built-in syntax highlighting for Rust code
- **Multiple File Support**: Work with multiple files simultaneously
- **Configuration System**: Customizable key bindings and settings

## License

This project is licensed under the GPL-3.0 License - see the [LICENSE](LICENSE.md) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

---

Created by 0l3d
