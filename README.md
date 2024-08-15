# concurent-image-processing

# 📸 Image Processor in Rust

Welcome to the **Image Processor** project! This Rust application is designed to efficiently convert images to grayscale and resize them using parallel processing. 🚀

## 🛠 Project Structure

```bash
my_project/
├── Cargo.toml        # Project dependencies and metadata
├── src/
│   └── main.rs       # Main application logic
├── images/           # Input images directory
│   ├── image1.png
│   ├── image2.jpg
│   └── ...
└── output/           # Output images directory (processed images saved here)


# 🛠 Building Rust Project with Sublime Text

This guide will help you set up and build your Rust project using Sublime Text. Follow these steps to integrate Rust with Sublime Text and run your project efficiently.

## 📂 Setting Up Sublime Text for Rust

### 1. Install Rust

Before setting up Sublime Text, ensure Rust is installed on your system. You can install Rust using [rustup](https://rustup.rs/).

### 2. Install Sublime Text

If you haven’t already, download and install [Sublime Text](https://www.sublimetext.com/).

### 3. Install Rust Package for Sublime Text

For better Rust support in Sublime Text, consider installing a Rust package from Package Control:

- Open Sublime Text.
- Go to **Preferences > Package Control**.
- Select **Install Package**.
- Search for and install the **Rust** package.

### 4. Create a Build System for Rust

To build and run your Rust project from Sublime Text, you need to create a custom build system:

1. Open Sublime Text.
2. Go to **Tools > Build System > New Build System**.

Replace the content with the following configuration:

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
