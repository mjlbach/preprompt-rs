use anyhow::{Context, Result};
use clap::Parser;
use clipboard::{ClipboardContext, ClipboardProvider};
use log::{info, warn};
use mime_guess::from_path;
use simple_logger::SimpleLogger;
use std::{fs,io};
use std::io::Read;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const DEFAULT_OUTPUT_FORMAT: &str = "markdown";

#[derive(clap::ValueEnum, Clone, Debug)]  
enum Level {  
   Debug,  
   Info,  
   Warning,  
   Error,  
}

// Conversion from custom Level to log::LevelFilter
impl From<Level> for log::LevelFilter {
    fn from(level: Level) -> Self {
        match level {
            Level::Debug => log::LevelFilter::Debug,
            Level::Info => log::LevelFilter::Info,
            Level::Warning => log::LevelFilter::Warn,
            Level::Error => log::LevelFilter::Error,
        }
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Cli {
    #[clap(help = "The path to the directory to traverse", value_parser(clap::value_parser!(PathBuf)))]
    path: PathBuf,
    #[clap(value_enum, default_value_t = Level::Warning)]
    log_level: Level,
    #[clap(short, long, default_value = DEFAULT_OUTPUT_FORMAT)]
    output_format: String,
}

fn setup_logger(level: Level) -> Result<()> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::from(level))
        .init()
        .context("Failed to set up logger")
}

fn read_file_to_string(file_path: &Path) -> io::Result<String> {
    let mut file = fs::File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn is_text_file(file_path: &Path) -> bool {
    from_path(file_path)
        .first()
        .map_or(false, |mime| mime.type_() == "text")
}

fn format_output<P: AsRef<Path>, S: AsRef<str>>(relative_path: P, contents: S, format: &str) -> Result<String> {
    match format {
        "markdown" => Ok(format!(
            "### {}\n```\n{}\n```\n",
            relative_path.as_ref().display(),
            contents.as_ref()
        )),
        "plain" => Ok(format!(
            "{}\n{}\n",
            relative_path.as_ref().display(),
            contents.as_ref()
        )),
        _ => Err(anyhow::anyhow!("Unsupported output format: {}", format)),
    }
}

fn copy_to_clipboard(content: &str) -> Result<()> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()
        .map_err(|e| anyhow::Error::msg(format!("Failed to create clipboard context: {}", e)))?;
    ctx.set_contents(content.to_owned())
        .map_err(|e| anyhow::Error::msg(format!("Failed to copy contents to clipboard: {}", e)))
}

fn main() -> Result<()> {
    let args = Cli::parse();
    setup_logger(args.log_level)?;

    let directory_path = fs::canonicalize(&args.path)
    .with_context(|| "Unable to find or access the specified directory path")?;

    if !directory_path.is_dir() {
        warn!("The path specified is not a directory.");
        return Ok(());
    }

    let mut clipboard_content = String::with_capacity(1024); // Pre-allocate some memory to reduce reallocation.
    for entry in WalkDir::new(&directory_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && is_text_file(e.path())) {
        let file_path = entry.path();
        let relative_path = file_path.strip_prefix(&directory_path).context("Failed to compute relative path")?;

        info!("Traversing file: {}", relative_path.display());

        let file_contents = read_file_to_string(file_path)?;
            let first_line = file_contents.lines().next().unwrap_or("File is empty or unreadable");
        info!("First line of {}: {}", relative_path.display(), first_line);
        clipboard_content.push_str(&format_output(relative_path, file_contents, &args.output_format)?);
    }

    copy_to_clipboard(&clipboard_content)?;

    info!("File contents copied to clipboard!");

    Ok(())
}
