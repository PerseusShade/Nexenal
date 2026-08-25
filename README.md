# Nexenal

The ultimate developer arsenal, built for speed and efficiency.

## Installation & Configuration

Nexenal uses a unified, `.gitignore`-style configuration system stored in `config.json`. Rules support glob patterns (`*.ext`, `folder/`, exact filenames) to ignore unwanted files or directories automatically, as well as reusable text replacement rules and custom command workflows.

To view your current global configuration:
> nexenal config view

---

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
- `-q, --quiet` : Run silently without console output.

*Example: `nexenal tree -d ./src -o map.txt -i components -c`*

---

### 2. All
Gathers and merges all files matching a specific extension (or `*` for all files) into a single context block. Perfect for code reviews or feeding context to LLMs.

**Usage:**
> nexenal all <extension>

**Options:**
- `-d, --dir <path>` : Target directory to scan (default is `.`).
- `-o, --output <file>` : Output file name (defaults to `merged_code.txt`).
- `-i, --ignore <pattern>` : Add extra ignore patterns.
- `-c, --clipboard` : Copy all merged code directly to clipboard.
- `-q, --quiet` : Run silently without console output.

*Example: `nexenal all rs -c`*

---

### 3. Replace
Performs batch text substitutions across project files using rules defined in `config.json` or ad-hoc arguments, while respecting your global ignore list.

**Usage:**
> nexenal replace [options]

**Options:**
- `-e, --ext <extension>` : Target file extension (defaults to `*`).
- `-d, --dir <path>` : Target directory to scan (default is `.`).
- `-i, --ignore <pattern>` : Add extra ignore patterns.
- `-s, --ruleset <name>` : Apply a predefined ruleset from `config.json` (e.g. `weird-chars`).
- `--rule <name>` : Apply a single rule from `config.json`.
- `-f, --find <string>` : Find a specific string directly (requires `-r, --replace`).
- `-r, --replace <string>` : Replacement string.
- `--dry-run` : Preview modified files and match counts without writing changes to disk.
- `-q, --quiet` : Run silently without printing modified files.

*Examples:*
```bash
nexenal replace -s weird-chars
nexenal replace -e rs -f "foo" -r "bar"
nexenal replace -s weird-chars --dry-run