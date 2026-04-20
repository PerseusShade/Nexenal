# Nexenal

The ultimate developer arsenal, built for speed and efficiency.

## Installation & Configuration

Nexenal uses a layered configuration system. Default rules can be overridden by global JSON settings, which can in turn be overridden by specific terminal commands.

To view your current global configuration:
> nexenal config view

## Commands

### 1. Tree
Generates a clean, visual representation of your project's architecture, automatically ignoring junk folders (`node_modules`, `.git`, etc.).

**Usage:**
> nexenal tree

**Options:**
- `-d, --dir <path>` : Target directory to scan (default is current folder `.`).
- `-o, --output <file>` : Specific output file name (overrides JSON config).
- `-i, --ignore <folder>` : Add an extra folder to ignore JUST for this run.

*Example: `nexenal tree -d ./src -o map.txt -i components`*

### 2. All
Gathers and merges all files of a specific extension into a single text file. Perfect for code review or feeding an entire project's context to an AI.

**Usage:**
> nexenal all <extension>

**Options:**
- `-d, --dir <path>` : Target directory to scan.
- `-o, --output <file>` : Specific output file name.
- `-i, --ignore <folder>` : Ignore specific folders.

*Example: `nexenal all js -o full_code.txt`*

### 3. Config
Manages the global `config.json` rules persistently.

**Commands:**
- `nexenal config view` : Displays the current JSON configuration.
- `nexenal config ignore <folder>` : Adds a folder to the permanent ignore list.
- `nexenal config unignore <folder>` : Removes a folder from the ignore list.

---
*Created by PerseusShade.*