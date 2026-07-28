# Nexenal

The ultimate developer arsenal, built for speed and efficiency.

## Installation & Configuration

Nexenal uses a unified, `.gitignore`-style configuration system stored in `config.json`. Rules support glob patterns (`*.ext`, `folder/`, exact filenames) to ignore unwanted files or directories automatically.

To view your current global configuration:
> nexenal config view

## Commands

### 1. Tree
Generates a clean, visual representation of your project's architecture, automatically ignoring files/folders defined in your config.

**Usage:**
> nexenal tree

**Options:**
- `-d, --dir <path>` : Target directory to scan (default is `.`).
- `-o, --output <file>` : Output file name (defaults to `struct.txt`).
- `-i, --ignore <pattern>` : Add extra ignore patterns just for this run.
- `-c, --clipboard` : Copy the visual tree directly to clipboard instead of saving to a file.

*Example: `nexenal tree -d ./src -o map.txt -i components -c`*

---

### 2. All
Gathers and merges all files matching a specific extension (or `*` for all files) into a single context block. Perfect for code reviews or feeding context to LLMs.

**Usage:**
> nexenal all <extension>

**Options:**
- `-d, --dir <path>` : Target directory to scan.
- `-o, --output <file>` : Output file name (defaults to `merged_code.txt`).
- `-i, --ignore <pattern>` : Add extra ignore patterns.
- `-c, --clipboard` : Copy all merged code directly to clipboard.

*Example: `nexenal all rs -c`*

---

### 3. Config
Manages global persistent rules in `config.json`.

**Commands:**
- `nexenal config view` : Displays current JSON configuration.
- `nexenal config edit` : Opens `config.json` in your default system editor.
- `nexenal config ignore <pattern>` : Adds a folder, file, or glob pattern to the global ignore list.
- `nexenal config unignore <pattern>` : Removes a pattern from the ignore list.
- `nexenal config reset` : Resets `config.json` to factory default settings.
- `nexenal config update` : Fetches and syncs the latest default configuration rules directly from GitHub.

---

### 4. Utility Commands
- `nexenal clean <target>` : Cleans generated files (`tree`, `all`, or `*` for both).
- `nexenal update` : Checks GitHub Releases and auto-updates the Nexenal binary to the latest version.
- `nexenal docs` : Opens the official `README.md`.
- `nexenal license` : Displays license information (`LICENSE.md`).

---
*Created by PerseusShade.*