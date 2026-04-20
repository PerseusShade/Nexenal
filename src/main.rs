use clap::{CommandFactory, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command as SysCommand;
use self_update::backends::github::Update;

#[derive(Parser)]
#[command(name = "nexenal")]
#[command(version = "1.0")]
#[command(about = "The ultimate developer arsenal", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generates a visual tree of your project architecture
    Tree {
        #[arg(short, long, default_value = ".")]
        dir: String,
        #[arg(short, long)]
        output: Option<String>,
        #[arg(short, long)]
        ignore: Vec<String>,
    },
    /// Merges all files with a specific extension into a single file
    All {
        ext: String,
        #[arg(short, long, default_value = ".")]
        dir: String,
        #[arg(short, long)]
        output: Option<String>,
        #[arg(short, long)]
        ignore: Vec<String>,
    },
    /// Manage Nexenal configuration (JSON)
    Config {
        #[command(subcommand)]
        action: ConfigActions,
    },
    /// Updates Nexenal to the latest version from GitHub
    Update,
    /// Opens the official documentation (README.md)
    Docs,
    /// Opens the software license (LICENSE.md)
    License,
}

#[derive(Subcommand)]
enum ConfigActions {
    View,
    Edit,
    Ignore { folder: String },
    Unignore { folder: String },
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct Config {
    #[serde(default)]
    global: GlobalConfig,
    #[serde(default)]
    tree: TreeConfig,
    #[serde(default)]
    all: AllConfig,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct GlobalConfig {
    #[serde(default)]
    ignore: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TreeConfig {
    default_output: String,
}
impl Default for TreeConfig {
    fn default() -> Self {
        Self { default_output: "struct.txt".to_string() }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct AllConfig {
    default_output: String,
}
impl Default for AllConfig {
    fn default() -> Self {
        Self { default_output: "merged_code.txt".to_string() }
    }
}

fn get_asset_path(filename: &str) -> PathBuf {
    if let Ok(mut exe_path) = env::current_exe() {
        exe_path.pop();
        exe_path.push(filename);
        return exe_path;
    }
    PathBuf::from(filename)
}

fn load_config() -> Config {
    let config_file = get_asset_path("config.json");

    if let Ok(content) = fs::read_to_string(&config_file) {
        if let Ok(config) = serde_json::from_str::<Config>(&content) {
            return config;
        } else {
            eprintln!("[WARNING] Failed to parse config.json. Using default settings.");
        }
    }
    Config::default()
}

fn save_config(config: &Config) -> io::Result<()> {
    let config_file = get_asset_path("config.json");
    let content = serde_json::to_string_pretty(config)?;
    fs::write(config_file, content)?;
    Ok(())
}

fn get_base_ignores(config: &Config) -> Vec<String> {
    let mut ignores = vec![
        ".git".to_string(),
        "__pycache__".to_string(),
        "target".to_string(),
        "node_modules".to_string(),
    ];
    ignores.extend(config.global.ignore.clone());
    ignores
}

fn open_file(filename: &str) {
    let path = get_asset_path(filename);

    if !path.exists() {
        eprintln!("[ERROR] File '{}' not found. Ensure it was deployed correctly.", filename);
        return;
    }

    println!("Opening {}...", filename);
    let status = SysCommand::new("cmd")
        .arg("/C")
        .arg("start")
        .arg("")
        .arg(&path)
        .spawn();

    if status.is_err() {
        eprintln!("[ERROR] Failed to open the file.");
    }
}

fn update_nexenal() -> Result<(), Box<dyn std::error::Error>> {
    println!("Checking for updates on GitHub...");

    let status = Update::configure()
        .repo_owner("PerseusShade")
        .repo_name("Nexenal")
        .bin_name("nexenal.exe")
        .show_download_progress(true)
        .current_version(env!("CARGO_PKG_VERSION"))
        .build()?
        .update()?;

    if status.updated() {
        println!("Success! Nexenal has been updated to version {}.", status.version());
    } else {
        println!("Nexenal is already up to date (v{}).", env!("CARGO_PKG_VERSION"));
    }

    Ok(())
}

fn run_tree(dir_path: &Path, prefix: &str, out: &mut String, ignore_dirs: &[String]) -> io::Result<()> {
    let mut entries: Vec<_> = fs::read_dir(dir_path)?.filter_map(Result::ok).collect();

    entries.retain(|entry| {
        let name = entry.file_name().to_string_lossy().to_string();
        let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
        is_dir || !ignore_dirs.contains(&name)
    });

    entries.sort_by(|a, b| {
        let a_is_dir = a.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
        let b_is_dir = b.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
        b_is_dir.cmp(&a_is_dir).then(a.file_name().cmp(&b.file_name()))
    });

    let count = entries.len();

    for (i, entry) in entries.iter().enumerate() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
        let is_last = i == count - 1;
        let connector = if is_last { "└── " } else { "├── " };

        if is_dir && ignore_dirs.contains(&name) {
            out.push_str(&format!("{}{}{}/\n", prefix, connector, name));
            continue;
        }

        let display_name = if is_dir { format!("{}/", name) } else { name.clone() };
        out.push_str(&format!("{}{}{}\n", prefix, connector, display_name));

        if is_dir {
            let new_prefix = if is_last { format!("{}    ", prefix) } else { format!("{}│   ", prefix) };
            run_tree(&path, &new_prefix, out, ignore_dirs)?;
        }
    }
    Ok(())
}

fn run_all(dir_path: &Path, target_ext: &str, out_file: &mut File, ignore_dirs: &[String], base_path: &Path) -> io::Result<()> {
    let entries: Vec<_> = fs::read_dir(dir_path)?.filter_map(Result::ok).collect();

    for entry in entries {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if ignore_dirs.contains(&name) {
            continue;
        }

        if path.is_dir() {
            run_all(&path, target_ext, out_file, ignore_dirs, base_path)?;
        } else if name.ends_with(&format!(".{}", target_ext)) {
            let relative_path = path.strip_prefix(base_path).unwrap_or(&path);

            writeln!(out_file, "{} :\n", relative_path.display())?;

            match fs::read_to_string(&path) {
                Ok(content) => writeln!(out_file, "{}", content)?,
                Err(e) => writeln!(out_file, "[READ ERROR: {}]", e)?,
            }

            writeln!(out_file, "\n{}\n", "=".repeat(50))?;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let mut config = load_config();
    let base_ignores = get_base_ignores(&config);

    match cli.command {
        Some(Commands::Tree { dir, output, ignore }) => {
            let root = Path::new(&dir);
            let final_output = output.unwrap_or(config.tree.default_output);

            let mut final_ignores = base_ignores.clone();
            final_ignores.extend(ignore);

            let mut tree_content = String::new();
            let abs_path = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());

            let mut clean_scan_path = abs_path.display().to_string();
            if clean_scan_path.starts_with(r"\\?\") {
                clean_scan_path = clean_scan_path.replacen(r"\\?\", "", 1);
            }

            let root_name = abs_path.file_name().unwrap_or_default().to_string_lossy();

            tree_content.push_str(&format!("{}/\n", root_name));

            println!("Nexenal [Tree] scanning: {}", clean_scan_path);
            run_tree(root, "", &mut tree_content, &final_ignores)?;

            let mut file = File::create(&final_output)?;
            file.write_all(tree_content.as_bytes())?;
            println!("Success! Architecture generated in '{}'.", final_output);
        }

        Some(Commands::All { ext, dir, output, ignore }) => {
            let root = Path::new(&dir);
            let final_output = output.unwrap_or(config.all.default_output);

            let mut final_ignores = base_ignores.clone();
            final_ignores.extend(ignore);

            let mut file = File::create(&final_output)?;

            let abs_path = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
            let mut clean_scan_path = abs_path.display().to_string();
            if clean_scan_path.starts_with(r"\\?\") {
                clean_scan_path = clean_scan_path.replacen(r"\\?\", "", 1);
            }

            println!("Nexenal [All] gathering '.{}' files from {}...", ext, clean_scan_path);
            run_all(root, &ext, &mut file, &final_ignores, root)?;

            println!("Success! Code merged into '{}'.", final_output);
        }

        Some(Commands::Config { action }) => {
            match action {
                ConfigActions::View => {
                    let json_output = serde_json::to_string_pretty(&config)?;
                    println!("--- Current Nexenal Configuration ---\n{}", json_output);
                }
                ConfigActions::Edit => {
                    open_file("config.json");
                }
                ConfigActions::Ignore { folder } => {
                    if !config.global.ignore.contains(&folder) {
                        config.global.ignore.push(folder.clone());
                        save_config(&config)?;
                        println!("Success! Added '{}' to global ignore list.", folder);
                    } else {
                        println!("Info: '{}' is already in the global ignore list.", folder);
                    }
                }
                ConfigActions::Unignore { folder } => {
                    if let Some(pos) = config.global.ignore.iter().position(|x| *x == folder) {
                        config.global.ignore.remove(pos);
                        save_config(&config)?;
                        println!("Success! Removed '{}' from global ignore list.", folder);
                    } else {
                        println!("Info: '{}' was not found in the global ignore list.", folder);
                    }
                }
            }
        }

        Some(Commands::Update) => {
            if let Err(e) = update_nexenal() {
                eprintln!("[ERROR] Update failed: {}", e);
            }
        }

        Some(Commands::Docs) => {
            open_file("README.md");
        }

        Some(Commands::License) => {
            open_file("LICENSE.md");
        }

        None => {
            let _ = Cli::command().print_help();
        }
    }

    Ok(())
}