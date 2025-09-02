# EDFI

A lightweight, vim-like modal terminal text editor written in Rust using Ratatui.

## Features

### Current Features

- **Modal Editing**: Vi-inspired dual-mode editing system
- **File Operations**: Create, open, and save files
- **Smart Cursor Movement**: Navigate efficiently through text
- **Command Line Integration**: Open files directly from terminal
- **Auto-scrolling**: Smooth horizontal and vertical scrolling when cursor moves beyond visible area
- **Line Management**: Automatic line creation, deletion, and merging
- **Backspace Handling**: Intelligent line merging and deletion
- **Visual Feedback**: Clear indication of current mode and file status
- **Line Deletion**: Delete entire lines in normal mode
- **Find Mode**: Search in the file.
- **Undo/Redo**: History management for editing operations.

### Modes

#### Normal Mode

- `i` - Enter editing mode
- `s` - Save current file
- `o` - Open/reload file (reloads content from the originally opened file)
- `d` - Delete current line
- `q` - Quit editor
- Arrow keys - Move cursor
- `Home` - Move to beginning of line
- `End` - Move to end of line

#### Editing Mode

- `ESC` - Return to normal mode
- `Enter` - Create new line
- `Backspace` - Delete character or merge lines intelligently
- Arrow keys - Move cursor
- `Home` - Move to beginning of line
- `End` - Move to end of line
- Any printable character - Insert at cursor position

#### Find Mode

- `ESC` - Return to normal mode
- Arrow keys - Move cursor
- Any printable character - Search in the file

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

This creates a file called "new_file" by default.

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

<img width="1120" height="693" alt="image" src="https://github.com/user-attachments/assets/56f764a1-5ecf-44dc-a1b7-2a064d6ba73f" />

_[ffetch](https://github.com/0l3d/ffetch) configuration with EDFI_

### Rust Code

<img width="1096" height="973" alt="image" src="https://github.com/user-attachments/assets/bf3e6aab-4891-4975-9d1b-b03216dea352" />

### Find

<img width="1023" height="972" alt="image" src="https://github.com/user-attachments/assets/fe04dce3-6965-4410-a653-b2221f99acd8" />

## Planned Features

- **Lua Script Support**: Extend editor functionality with Lua scripting
- **Enhanced Syntax Highlighting**: Support for more programming languages
- **Multiple File Support**: Work with multiple files simultaneously
- **Configuration System**: Customizable key bindings and settings

## License

This project is licensed under the GPL-3.0 License - see the [LICENSE](LICENSE.md) file for details.

## Author

Created by **0l3d**
