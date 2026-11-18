<div align="center">

```
  __  __ _____ __  __  ___
 |  \/  | ____|  \/  |/ _ \
 | |\/| |  _| | |\/| | | | |
 | |  | | |  | | |_| |
 |_|  |_|_____|_|  |_|\___/
```

**Capture your thoughts. Stay in the flow.**

[![CI](https://github.com/epxmdev/memo/actions/workflows/ci.yml/badge.svg)](https://github.com/epxmdev/memo/actions/workflows/ci.yml)
![Rust](https://img.shields.io/badge/rust-1.85%2B-orange)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Release](https://img.shields.io/github/v/release/epxmdev/memo?include_prereleases)](https://github.com/epxmdev/memo/releases)

**A lightning-fast, local-first interactive note-taking utility for your terminal.**

</div>

Memo is a minimalist note-taking tool built in Rust. It opens a responsive terminal REPL where you can jot down ideas, search past entries, and manage your notes without switching apps.

Everything runs locally. No telemetry, no account, and no cloud synchronization.

## Features

- **Interactive REPL** — Launch `memo` and type commands at the `memo> ` prompt.
- **Persistent command history** — Browse previous commands across sessions.
- **Keyword search** — Find notes with a case-insensitive text search.
- **Local JSON storage** — Notes are stored in `~/.local/share/memo/notes.json`.
- **Automatic timestamps and IDs** — Notes receive a local timestamp and sequential ID.
- **Colorized terminal output** — Readable tables and helpful status messages.
- **Offline by default** — Your notes stay on your machine.

## Installation

### Requirements

- Rust 1.85 or newer
- Cargo
- Git
- A C compiler and standard build tools

### CachyOS / Arch Linux

Install the build tools:

```bash
sudo pacman -Syu
sudo pacman -S --needed base-devel git
```

Check your tools:

```bash
rustc --version
cargo --version
git --version
```

If Rust is missing, install it using [rustup](https://rustup.rs/).

### Build from source

Clone the repository:

```bash
git clone https://github.com/epxmdev/memo.git
cd memo
```

Build and install:

```bash
cargo install --path .
```

Make sure Cargo's binary directory is in your `PATH`:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Add this line to `~/.bashrc` or `~/.zshrc` to keep it permanently.

Verify the installation:

```bash
memo --version
memo --help
```

## First launch

### 1. Start the interactive terminal

Run:

```bash
memo
```

This opens the `memo> ` prompt and creates the local data directory if it does not exist.

### 2. Add notes

Inside the prompt:

```text
memo> add Buy groceries after work
memo> add Call the dentist tomorrow at 10 AM
memo> add Project idea: build a local file-sharing tool in Rust
```

### 3. Browse and search

```text
memo> list
memo> search Rust
memo> rm 2
memo> exit
```

## Command reference

| Command | Description |
|---|---|
| `add <text>` | Create a timestamped note |
| `list` | Display all saved notes with IDs and dates |
| `search <keyword>` | Find notes whose text contains the keyword |
| `rm <id>` | Delete a note by ID and re-index the remaining notes |
| `help` | Display the command reference |
| `exit` / `quit` | Exit the interactive session |

You can also press `Ctrl+C` or `Ctrl+D` to exit.

## Example

```text
memo> add Finish the Rust CLI documentation
OK: Note 1 added.

memo> list
+----+---------------------+----------------------------------+
| ID | DATE                | NOTE                             |
+----+---------------------+----------------------------------+
|  1 | 2026-11-18 16:45:00 | Finish the Rust CLI documentation |
+----+---------------------+----------------------------------+
1 note(s)

memo> search documentation
1 result(s) for "documentation"

memo> rm 1
OK: Note 1 deleted.
```

The application displays its status messages in French; the example above is translated for readability.

## Configuration and privacy

Memo stores its data locally:

- **Notes:** `~/.local/share/memo/notes.json`
- **Command history:** `~/.local/share/memo/history.txt`

Your notes never leave your computer. You can back up or edit the JSON file with any standard tool.

## Troubleshooting

### `memo: command not found`

Make sure Cargo's binary directory is in your `PATH`:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

### Build errors

Check your Rust version and build tools:

```bash
rustc --version
cargo --version
gcc --version
```

On CachyOS / Arch Linux:

```bash
sudo pacman -S --needed base-devel git
```

Then retry:

```bash
cargo install --path . -v
```

### Cannot save notes (Permission denied)

Ensure your user has write permissions to `~/.local/share/`:

```bash
mkdir -p ~/.local/share/memo
chmod 755 ~/.local/share/memo
```

## Development

Build the project:

```bash
cargo build
```

Run it from source:

```bash
cargo run
```

Run tests:

```bash
cargo test --all-targets
```

## Architecture

Memo is built in Rust with:

- `rustyline` for the interactive REPL and persistent line history
- `serde` and `serde_json` for local JSON storage
- `chrono` for timestamps
- `colored` for terminal styling
- `dirs` to locate the user's home directory

## Roadmap

- Tagging support (`#work`, `#personal`, `#ideas`)
- Note editing (`edit <id>`)
- Markdown and plain-text export
- Optional due-date reminders with desktop notifications

## Contributing

Contributions are welcome. Feel free to open an issue or submit a pull request on [GitHub](https://github.com/epxmdev/memo).

## License

Memo is distributed under the MIT License. See [LICENSE](LICENSE).

---

<div align="center">

**Built with Rust. Made for everyday productivity.**

[GitHub Repository](https://github.com/epxmdev/memo)

</div>
