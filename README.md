# Project Setup and Build Instructions

## Setting Up Sublime Text for Rust

### 1. Install Rust

Make sure Rust is installed. If not, install it using [rustup](https://rustup.rs/).

### 2. Install Sublime Text

Download and install [Sublime Text](https://www.sublimetext.com/).

### 3. Install Rust Package for Sublime Text

To add Rust support:
1. Open Sublime Text.
2. Go to **Preferences > Package Control**.
3. Select **Install Package**.
4. Search for **Rust** and install it.

### 4. Create a Build System for Rust

1. Open Sublime Text.
2. Navigate to **Tools > Build System > New Build System**.

Replace the content with the following:

```json
{
    "shell_cmd": "cargo build",
    "working_dir": "${folder:${project_path:${file_path}}}",
    "selector": "source.rust",
    "variants": [
        {
            "name": "Build and Run",
            "shell_cmd": "cargo run"
        },
        {
            "name": "Build",
            "shell_cmd": "cargo build"
        },
        {
            "name": "Test",
            "shell_cmd": "cargo test"
        },
        {
            "name": "Check",
            "shell_cmd": "cargo check"
        },
        {
            "name": "Clean",
            "shell_cmd": "cargo clean"
        }
    ]
}

![Example Input Image]()
