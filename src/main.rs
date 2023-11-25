use std::fs;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};
use clipboard::{ClipboardProvider, ClipboardContext};
use structopt::StructOpt;
use mime_guess::from_path;

#[derive(StructOpt)]
struct Cli {
    /// The path to the directory to copy
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn is_text_file(file_name: &str) -> bool {
    from_path(file_name).first().map_or(false, |mime| mime.type_() == "text")
}

fn main() {
    let args = Cli::from_args();
    let directory_path = fs::canonicalize(args.path).expect("Failed to convert path to absolute path");

    // Check if the given path is a directory
    if !directory_path.is_dir() {
        eprintln!("The path specified is not a directory.");
        return;
    }

    let mut clipboard_content = String::new();

    // Recursively walk through the directory
    for entry in WalkDir::new(&directory_path)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let file_name = entry.file_name().to_string_lossy();
        if is_text_file(&file_name) {
            let relative_path = entry.path().strip_prefix(&directory_path).unwrap().to_string_lossy();
            let file_contents = fs::read_to_string(entry.path())
                .unwrap_or_else(|_| String::from("Error reading file"));

            // Append the file content with markdown header
            clipboard_content.push_str(&format!("### {}\n```\n{}\n```\n", relative_path, file_contents));
        }
    }

    // Copy the collected data to the clipboard
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(clipboard_content).expect("Failed to copy to clipboard");

    println!("File contents copied to clipboard!");
}
