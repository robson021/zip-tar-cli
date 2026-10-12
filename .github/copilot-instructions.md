# Repository guidance

## Build, test, and format

- Build: `cargo build`
- Run the test suite: `cargo test`
- Run a focused test by name, for example: `cargo test parse_zip`
- Check formatting: `cargo fmt --check`

These are the same build, test, and formatting checks used by `.github/workflows/rust.yml`. Tests are currently inline in the Rust modules; there is no separate lint command configured.

## Architecture

This is a Rust application that builds and executes shell commands for system `tar` and `zip` utilities rather than handling archive data through Rust libraries.

- `main.rs` checks for `--gui` (delegating to `gui::run_gui()`), falls back to `cmd::arg_parser` if arguments are provided, or starts the interactive terminal menu loop via `input_handler`.
- `gui.rs` provides an `eframe`/`egui` desktop interface with native file dialogs (`rfd`) for unpacking archives, creating ZIP or TAR archives from files or directories, and configuring password encryption. It reuses `file_metadata`, `command_builder`, and `command_runner`.
- `cmd/arg_parser.rs` validates the three process arguments (program, action, path), resolves path metadata, and maps action aliases to builder functions.
- `cmd/command_builder.rs` turns paths and metadata into shell-command strings for extraction, bulk extraction, ZIP, and TAR operations.
- `cmd/command_runner.rs` executes those strings with `sh -c`; Windows is currently unsupported.
- `file/file_metadata.rs` resolves a path into `FileMetadata`, distinguishing a file from a directory and retaining a trailing wildcard. `file/string_utils.rs` contains the path/wildcard helpers.
- `input_handler.rs` supplies the interactive prompts and uses the same metadata layer as argument mode. `error.rs` defines the shared operation errors.

## Repository-specific conventions

- Keep CLI action aliases and their dispatch in `arg_parser.rs`; keep command construction in `command_builder.rs` so interactive and argument-driven operations share the same behavior.
- `FileMetadata` represents a wildcard input as its containing directory in `path` plus the original pattern in `wildcard`. Use `to_string_path()` when a builder needs the file selection string, and `to_short_name()` when deriving an output archive name.
- Builder functions return command strings; the runner is the execution boundary. Unit tests commonly assert exact command strings and use the fixtures under `resources/test/`.
- Bulk extraction filters paths by the extension list in `VALID_ARCHIVE_FORMATS` in `command_builder.rs`; keep that list aligned with the formats supported by the CLI.
