use anyhow::{Context, Result};
use clipboard::{ClipboardContext, ClipboardProvider};
use ignore::WalkBuilder;
use log::{info, warn};
use mime_guess::from_path;
use simple_logger::SimpleLogger;
use std::fs;
use std::io::{self, BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

const DEFAULT_LOG_LEVEL: &str = "info";
const DEFAULT_OUTPUT_FORMAT: &str = "markdown";
const CUSTOM_IGNORE_FILENAME: &str = ".myignore";

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: PathBuf,
    #[structopt(short, long, default_value = DEFAULT_LOG_LEVEL)]
    log_level: log::LevelFilter,
    #[structopt(short, long, default_value = DEFAULT_OUTPUT_FORMAT)]
    output_format: String,
}

fn is_text_file(file_path: &Path) -> bool {
    from_path(file_path)
        .first()
        .map_or(false, |mime| mime.type_() == "text")
}

fn read_file_to_string(file_path: &Path) -> io::Result<String> {
    fs::File::open(file_path).and_then(|file| {
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        Ok(contents)
    })
}

fn read_first_line(file_path: &Path) -> io::Result<String> {
    let file = fs::File::open(file_path)?;
    let mut lines = BufReader::new(file).lines();
    match lines.next() {
        Some(line) => line,
        None => Ok(String::new()),
    }
}

fn setup_logger(level: log::LevelFilter) -> Result<()> {
    SimpleLogger::new().with_level(level).init().context("Failed to set up logger")
}

fn format_output(relative_path: &Path, contents: &str, format: &str) -> Result<String> {
    match format {
        "markdown" => Ok(format!(
            "### {}\n```\n{}\n```\n",
            relative_path.display(),
            contents
        )),
        "plain" => Ok(format!("{}\n{}\n", relative_path.display(), contents)),
        _ => Err(anyhow::anyhow!("Unsupported output format: {}", format)),
    }
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    setup_logger(args.log_level)?;

    let directory_path = fs::canonicalize(&args.path).context("Failed to canonicalize path")?;
    if !directory_path.is_dir() {
        warn!("The path specified is not a directory.");
        return Ok(());
    }

    let mut clipboard_content = String::new();
    let walker = WalkBuilder::new(&directory_path)
        .add_custom_ignore_filename(CUSTOM_IGNORE_FILENAME)
        .build();

    for entry in walker.filter_map(|e| e.ok()).filter(|e| e.file_type().map_or(false, |ft| ft.is_file())) {
        // Extract relative path from entry
        let relative_path = entry.path().strip_prefix(&directory_path)?;
        info!("Traversing file: {}", relative_path.display());

        // Check if the file is a text file
        if is_text_file(entry.path()) {
            match read_first_line(entry.path()) {
                Ok(first_line) => info!("First line of {}: {}", relative_path.display(), first_line),
                Err(e) => {
                    warn!("Skipping file {}: {}", relative_path.display(), e);
                    continue;
                }
            }
            let file_contents = read_file_to_string(entry.path())?;
            clipboard_content.push_str(&format_output(relative_path, &file_contents, &args.output_format)?);
        } else {
            info!("Skipping non-text file: {}", relative_path.display());
        }
    }


        // Create a new clipboard provider and handle the error.
    let mut ctx: ClipboardContext = ClipboardProvider::new().map_err(|e|
        anyhow::Error::msg(format!("Failed to create clipboard context: {}", e))
    )?;

    // Set the clipboard contents and handle the error.
    ctx.set_contents(clipboard_content).map_err(|e|
        anyhow::Error::msg(format!("Failed to copy contents to clipboard: {}", e))
    )?;
    
    info!("File contents copied to clipboard!");

    Ok(())
}
