# tempo: The Code Templating Assistant

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
<!-- Add other badges later, e.g., for build status 
[![Build Status](https://img.shields.io/github/actions/workflow/status/YabetsZ/tempo/build.yml?branch=main)](https://github.com/YabetsZ/tempo/actions) -->

`tempo` is a command-line interface (CLI) application designed to help you quickly manage and use code templates for various purposes, especially useful for competitive programming, project scaffolding, or any repetitive coding tasks.
## Table of Contents

- [tempo: The Code Templating Assistant](#tempo-the-code-templating-assistant)
  - [Table of Contents](#table-of-contents)
  - [Overview](#overview)
  - [Features](#features)
  - [Installation](#installation)
    - [Prerequisites](#prerequisites)
    - [Option 1: From Source (Recommended for Developers)](#option-1-from-source-recommended-for-developers)
    - [Option 2: Pre-compiled Binaries (from GitHub Releases)](#option-2-pre-compiled-binaries-from-github-releases)
  - [Usage](#usage)
    - [Examples](#examples)
    - [Global Options](#global-options)
  - [Configuration](#configuration)
  - [Contributing](#contributing)
  - [License](#license)
## Overview

Ever find yourself copying and pasting the same boilerplate code or utility functions for every new problem (e.g., competitive programming) or project? `tempo` streamlines this by allowing you to store, manage, and apply your code templates with simple commands.

**Core Philosophy:**
*   **User-centric:** Easy to use, intuitive commands.
*   **Portable:** Templates stored in a user-specific directory, accessible from anywhere.
*   **Flexible:** Not tied to specific languages or project structures.
*   **Minimalist:** Does one thing well – managing and applying code templates.

## Features

*   **Add Templates:** `tempo add <name> <source_file_path>` - Store a new template.
*   **Apply Templates:** `tempo apply <name> <destination_path> [options]` - Create a new file or modify an existing one using a template.
    *   Strategies for existing files: overwrite (`-o`), append (`-a`), prepend (`-p`).
*   **List Templates:** `tempo list` (or `ls`) - View all your stored templates.
*   **Remove Templates:** `tempo remove <name>` (or `rm`) - Delete a template.
*   **Show Template Content:** `tempo show <name>` - Print a template's content to the console.
*   **Edit Templates:** `tempo edit <name>` - Open a template in your default editor.
*   **Show Template Path:** `tempo path <name>` - Display the full path to a stored template file.
*   **Verbose & Quiet Modes:** Control output with `-v`/`--verbose` and `-q`/`--quiet`.
*   **Force Option:** `-f`/`--force` to bypass confirmations or overwrite.
*   **Manifest-based:** Uses a `manifest.toml` file for robust template metadata management.

## Installation

### Prerequisites
*   [Rust programming language](https://www.rust-lang.org/tools/install) (includes `cargo`).

### Option 1: From Source (Recommended for Developers)
1.  Clone the repository:
    ```bash
    git clone https://github.com/<your_username>/tempo.git
    cd tempo
    ```
2.  Build and install the binary:
    ```bash
    cargo install --path .
    ```
    This will install `tempo` into `~/.cargo/bin/`. Ensure `~/.cargo/bin/` is in your system's `PATH` environment variable. The Rust installer usually helps set this up.

### Option 2: Pre-compiled Binaries (from GitHub Releases)

For each release, pre-compiled binary is provided for Windows (coming soon for macOS, and Linux) on the [GitHub Releases page](https://github.com/YabetsZ/tempo/releases).

1.  **Download** the appropriate archive or executable for your operating system and architecture from the latest release.
2.  **Extract** the `tempo` executable if it's in an archive (e.g., `.zip` or `.tar.gz`).
3.  **Place the `tempo` executable in a directory that is listed in your system's `PATH` environment variable.**
    This makes `tempo` accessible from any location in your terminal.

    Common locations:
    <!-- *   **Linux/macOS:**
        *   `~/.local/bin` (You might need to create this directory and add it to your PATH if it's not there by default).
        *   `/usr/local/bin` (Usually requires sudo/admin privileges). -->
    *   **Windows:**
        *   Create a folder like `C:\Program Files\tempo` or `C:\Users\<YourName>\bin` and add this folder to your user or system `PATH` environment variable.

    **How to add a directory to your PATH:**
    <!-- *   **Linux/macOS (Bash/Zsh):**
        Edit your shell's configuration file (e.g., `~/.bashrc`, `~/.zshrc`, `~/.profile`):
        ```bash
        # Add this line, replacing /path/to/your/tempo_directory with the actual path
        export PATH="/path/to/your/tempo_directory:$PATH"
        ```
        Then, either source the file (e.g., `source ~/.bashrc`) or open a new terminal session.
        If you chose `~/.local/bin`, you might add:
        ```bash
        if [ -d "$HOME/.local/bin" ] ; then
            PATH="$HOME/.local/bin:$PATH"
        fi
        ```-->
    *   **Windows:**
        1.  Search for "environment variables" in the Windows search bar.
        2.  Click on "Edit the system environment variables" (or "Edit environment variables for your account").
        3.  In the System Properties window, click the "Environment Variables..." button.
        4.  Under "User variables" (for just your account) or "System variables" (for all users), find the variable named `Path` and select it.
        5.  Click "Edit...".
        6.  Click "New" and add the full path to the directory where you placed `tempo.exe` (e.g., `C:\Users\<YourName>\bin`).
        7.  Click "OK" on all dialogs. You may need to open a new Command Prompt or PowerShell window for the changes to take effect.

4.  **Verify installation:**
    Open a new terminal/command prompt and type:
    ```bash
    tempo --version
    ```
    You should see the installed version of `tempo`.

## Usage

All `tempo` commands follow the pattern `tempo [GLOBAL_OPTIONS] <COMMAND> [ARGS]`.
You can get help for any command by running `tempo <COMMAND> --help`.

### Examples

**1. Adding a new template:**
```bash
# Create a Python template file (e.g., my_fast_io.py)
# Content of my_fast_io.py:
# import sys
# def solve():
#     input = sys.stdin.readline
#     # ... your code ...
# solve()

tempo add py_io ./my_fast_io.py
```

**2. Applying a template to a new file:**
```bash
tempo apply py_io solution.py
# This creates solution.py with the content of the 'py_io' template.
```

**3. Applying a template, appending to an existing file:**
```bash
echo "# My existing code" > main.cpp
tempo add cpp_utils ./my_utils.cpp # Assuming cpp_utils template exists
tempo apply cpp_utils main.cpp -a # Append
```

**4. Listing all templates:**
```bash
tempo list # OR tempo ls
```
Expected output:
```
Available templates:
--------------------
    - py_io (.py)
    - cpp_utils (.cpp)
```

**5. Showing a template's content:**
```bash
tempo show py_io
```

**6. Editing a template:**
```bash
tempo edit py_io
# This will open ~/.config/tempo/templates/py_io.py (or similar) in your default editor.
```

**7. Removing a template:**
```bash
tempo remove py_io
# Or with force:
tempo remove py_io --force
```

**8. Getting the stored path of a template:**
```bash
tempo path py_io
```

### Global Options
*   `-f, --force`: Overwrite existing files/templates or skip confirmations.
*   `-v, --verbose`: Enable verbose output for debugging or more details.
*   `-q, --quiet`: Suppress informational output (errors will still be shown).
*   `-h, --help`: Display help information.
*   `-V, --version`: Display application version.

## Configuration

`tempo` stores its templates and manifest file in a user-specific configuration directory:
*   **Linux/macOS[coming soon]:** `~/.config/tempo/`
*   **Windows:** `%APPDATA%\tempo\` (e.g., `C:\Users\<YourName>\AppData\Roaming\tempo\`)

The main components are:
*   `~/.config/tempo/templates/`: Directory containing the actual template files.
*   `~/.config/tempo/manifest.toml`: Metadata file for all stored templates.

## Contributing
Contributions are welcome! Please feel free to open an issue or submit a pull request.

## License
This project is licensed under the [MIT License](./LICENSE).
