# PrePrompt-RS

PrePrompt-RS is a Rust command-line tool designed to recursively read text files from a specified directory, excluding common build directories like `target` and `build`, and copy their contents to the clipboard in a Markdown format.

## Features

- Recursively reads text files from a given directory path.
- Skips files in directories typically ignored by version control (e.g., `.gitignore`).
- Copies contents of text files to the clipboard, formatted as Markdown sections.
- Handles errors gracefully, reporting files that could not be read.

## Installation

### From Source

Before the project is published to crates.io, you can install it directly from the source using Cargo. First, clone the repository:

```sh
git clone https://github.com/mjlbach/preprompt-rs.git
cd preprompt-rs
```

Then, install the binary `prep` using Cargo:

```sh
cargo install --path .
```

This will install the `prep` binary to your Cargo bin directory, which should be in your system's PATH.

### From crates.io

After the project is published to crates.io, you can install it using Cargo with the following command:

```sh
cargo install preprompt-rs
```

## Usage

Once installed, you can use the `prep` command followed by the path to the directory you want to process:

```sh
prep /path/to/directory
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
