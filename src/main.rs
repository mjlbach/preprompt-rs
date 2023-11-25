use std::{
    error::Error,
    fs,
    io::{self, BufReader},
    path::{Path, PathBuf},
};
use clipboard::{ClipboardProvider, ClipboardContext};
use ignore::WalkBuilder;
use structopt::StructOpt;
use mime_guess::from_path;
use std::io::prelude::*;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn is_text_file(file_path: &Path) -> bool {
    from_path(file_path).first().map_or(false, |mime| mime.type_() == "text")
}

fn read_file_to_string(file_path: &Path) -> io::Result<String> {
    let file = fs::File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();
    let directory_path = fs::canonicalize(args.path)?;

    if !directory_path.is_dir() {
        eprintln!("The path specified is not a directory.");
        return Ok(());
    }

    let mut clipboard_content = String::new();
    let walker = WalkBuilder::new(&directory_path)
        .add_custom_ignore_filename(".myignore")
        .build();

    for result in walker {
        let entry = result?;
        if entry.file_type().map_or(false, |ft| ft.is_file()) && is_text_file(entry.path()) {
            let relative_path = entry.path().strip_prefix(&directory_path)?.to_string_lossy();
            match read_file_to_string(entry.path()) {
                Ok(file_contents) => {
                    clipboard_content.push_str(&format!("### {}\n```\n{}\n```\n", relative_path, file_contents));
                },
                Err(e) => eprintln!("Error reading file {:?}: {}", relative_path, e),
            }
        }
    }

    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(clipboard_content)?;

    println!("File contents copied to clipboard!");

    Ok(())
}
