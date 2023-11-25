# PrePrompt-RS

PrePrompt-RS is a Rust command-line tool designed to recursively read text files from a specified directory, excluding common build directories like `target` and `build`, and copy their contents to the clipboard in a Markdown format.

## Features

- Recursively reads text files from a given directory path.
- Skips files in directories typically ignored by version control (e.g., `.gitignore`).
- Copies contents of text files to the clipboard, formatted as Markdown sections.
- Handles errors gracefully, reporting files that could not be read.

## Installation

To install PrePrompt-RS, you need to have Rust and Cargo installed on your machine. If you don't have them installed, you can get them from [the official Rust website](https://www.rust-lang.org/learn/get-started).

Once Rust and Cargo are installed, you can clone the repository and build the project:

```sh
git clone https://github.com/your-username/preprompt-rs.git
cd preprompt-rs
cargo build --release
```

The compiled binary will be located at `target/release/preprompt-rs`.

## Usage

To use PrePrompt-RS, simply run the binary with the path to the directory you want to process:

```sh
./target/release/preprompt-rs /path/to/directory
```

The contents of the text files will be copied to your clipboard in Markdown format.

## Custom Ignore Patterns

PrePrompt-RS respects ignore patterns defined in `.gitignore` files. If you have additional ignore patterns, you can create a `.myignore` file in the root of the directory being processed with your custom patterns.

Example `.myignore` content:

```
# Ignore all files in the 'logs' directory
logs/
```

## Contributing

Contributions are welcome! Please feel free to submit a pull request or create an issue if you have any ideas, questions, or find a bug.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
