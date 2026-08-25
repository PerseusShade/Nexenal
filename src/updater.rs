use crate::config::get_asset_path;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use self_update::backends::github::Update;
use std::process::Command as SysCommand;

pub fn copy_to_clipboard(content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx = ClipboardContext::new()?;
    ctx.set_contents(content.to_owned())?;
    Ok(())
}

pub fn open_file(filename: &str) {
    let path = get_asset_path(filename);

    if !path.exists() {
        eprintln!(
            "[ERROR] File '{}' not found. Ensure it was deployed correctly.",
            filename
        );
        return;
    }

    println!("Opening {}...", filename);
    let _ = SysCommand::new("cmd")
        .arg("/C")
        .arg("start")
        .arg("")
        .arg(&path)
        .spawn();
}

pub fn update_nexenal() -> Result<(), Box<dyn std::error::Error>> {
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
        println!(
            "Success! Nexenal has been updated to version {}.",
            status.version()
        );
    } else {
        println!(
            "Nexenal is already up to date (v{}).",
            env!("CARGO_PKG_VERSION")
        );
    }

    Ok(())
}