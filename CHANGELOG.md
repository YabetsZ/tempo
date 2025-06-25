# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.1.0-alpha.1] - 2025-06-17 

### Added
- Initial alpha release of `tempo`.
- Core template management functionalities:
    - `tempo add <name> <source_file_path>`: Adds a new code template.
    - `tempo apply <template_name> <destination_path> [options]`: Applies a template to a destination. Supports overwrite (`-o`), append (`-a`), and prepend (`-p`) strategies for existing files. (Formerly `new`).
    - `tempo list` (alias `ls`): Lists all stored templates.
    - `tempo remove <template_name>` (alias `rm`): Deletes a template.
    - `tempo show <template_name>`: Prints the content of a template to stdout.
    - `tempo edit <template_name>`: Opens a template in the default system editor.
    - `tempo path <template_name>`: Prints the full filesystem path to a stored template file.
- Manifest System:
    - Templates and their metadata (filename in storage, creation/update timestamps, source extension) are managed via `manifest.toml` located in the application's config directory.
    - Commands `add`, `list`, `remove`, `apply`, `show`, `edit`, `path` are all manifest-aware.
- Global Flags:
    - `-f, --force`: Bypasses confirmations (e.g., for `remove`) or forces overwrites (e.g., for `apply` when no strategy is specified and destination exists).
    - `-v, --verbose`: Enables more detailed output.
    - `-q, --quiet`: Suppresses informational output (errors are still shown). Verbose and quiet are mutually exclusive.
- Error Handling: Centralized `AppError` type for consistent error reporting.
- Output Management: `OutputConfig` struct for handling verbose/quiet printing.
- Basic project structure with `main.rs`, `cli.rs`, `config.rs`, `manifest.rs`, `error.rs`, `utils.rs`, and a `commands/` module.
