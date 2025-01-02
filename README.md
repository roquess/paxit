 # Paxit

Paxit is a flexible Rust library for compressing and decompressing files using various algorithms, with a modular design that allows easy addition of new compression formats.

## Features

- Supports multiple compression formats: ZIP, TAR, and GZIP.
- Modular design for easy addition of new compression algorithms.
- Simple command-line interface for compressing and decompressing files.

## Installation

To use Paxit, add the following to your `Cargo.toml`:

```toml
[dependencies]
paxit = "0.1.0"
```

## Usage

Command-Line Interface
Paxit provides a simple command-line interface for compressing and decompressing files.

### Compression

To compress files, use the following command:

```bash
paxit compress <output_file> <input_files...>
```

For example, to compress file1.txt and file2.txt into archive.zip:


```bash
paxit compress archive.zip file1.txt file2.txt
```

### Decompression

To decompress a file, use the following command:

```bash
paxit uncompress <input_file> <output_directory>
```

For example, to decompress archive.zip into the output directory:

```bash
paxit uncompress archive.zip output
```

## Library Usage

You can also use Paxit as a library in your Rust projects. Here is an example of how to use it:

```rust
use paxit::algorithms::{Zip, Tar, Gunzip};
use paxit::pack::Pack;
use std::path::Path;

fn main() {
    let files = vec![Path::new("file1.txt"), Path::new("file2.txt")];
    let output = Path::new("archive.zip");

    let compressor = Zip;
    compressor.pack(files, output).unwrap();
}
```

## Adding New Compression Algorithms

To add a new compression algorithm, simply create a new file in the algorithms directory and implement the Pack trait. The library will automatically detect and use the new algorithm.

# Contributing

Contributions are welcome! Please open an issue or submit a pull request.

# License

This project is licensed under the MIT License.
