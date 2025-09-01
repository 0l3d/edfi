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

![image](https://github.com/user-attachments/assets/045df880-4f64-4c44-94cd-b9ab68a0c39e)
_[ffetch](https://github.com/0l3d/ffetch) configuration with EDFI_

### Rust Code

![image](https://github.com/user-attachments/assets/30d39b31-6844-448b-bd3d-5b377ee8e529)

### Find

<img width="931" height="985" alt="image" src="https://github.com/user-attachments/assets/a587c1b6-3164-4980-a5df-81294e0e8cc2" />

## Planned Features

- **Lua Script Support**: Extend editor functionality with Lua scripting
- **Enhanced Syntax Highlighting**: Support for more programming languages
- **Multiple File Support**: Work with multiple files simultaneously
- **Configuration System**: Customizable key bindings and settings
- **Undo/Redo**: History management for editing operations
- **Line Numbers**: Optional line numbering display

## License

This project is licensed under the GPL-3.0 License - see the [LICENSE](LICENSE.md) file for details.

## Author

Created by **0l3d**
