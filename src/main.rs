use std::env;
use std::path::Path;
use paxit::algorithms::{Zip, Tar, Jpeg, Zstd, Lzma};
use paxit::pack::Pack;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: paxit <mode> <files...>");
        return Ok(());
    }

    let mode = &args[1];
    let files: Vec<&Path> = args[2..].iter().map(|s| Path::new(s)).collect();

    let compressor = match determine_compressor(mode, &files) {
        Ok(compressor) => compressor,
        Err(e) => {
            eprintln!("{}", e);
            return Ok(());
        }
    };

    match mode.as_str() {
        "c" | "compress" | "p" | "pack" => {
            let output = files[0];
            let input_files = files[1..].to_vec();
            compressor.pack(input_files, output)
        }
        "u" | "uncompress" | "unpack" => {
            let file = files[0];
            let output_dir = files[1];
            compressor.unpack(file, output_dir)
        }
        _ => Ok(()),
    }
}

fn determine_compressor(mode: &str, files: &[&Path]) -> Result<Box<dyn Pack>, String> {
    let extension = match mode {
        "c" | "compress" | "p" | "pack" => files[0].extension().and_then(|ext| ext.to_str()),
        "u" | "uncompress" | "unpack" => files[0].extension().and_then(|ext| ext.to_str()),
        _ => None,
    };

    match extension {
        Some("zip") => Ok(Box::new(Zip)),
        Some("tar") => Ok(Box::new(Tar)),
        Some("jpg" | "jpeg") => Ok(Box::new(Jpeg)),
        Some("zst") => Ok(Box::new(Zstd)),
        Some("xz") => Ok(Box::new(Lzma)),
        _ => Err(format!("Unsupported file format. Use .zip, .tar, .jpg/.jpeg, .zst, or .xz")),
    }
}

