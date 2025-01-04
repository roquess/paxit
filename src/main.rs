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

    let compressors = vec![
        Box::new(Zip) as Box<dyn Pack>,
        Box::new(Tar) as Box<dyn Pack>,
        Box::new(Jpeg) as Box<dyn Pack>,
        Box::new(Zstd) as Box<dyn Pack>,
        Box::new(Lzma) as Box<dyn Pack>,
    ];

    for compressor in compressors {
        if compressor.extensions().contains(&extension.unwrap_or("")) {
            return Ok(compressor);
        }
    }

    let supported_extensions: Vec<&str> = vec![
        Zip.extensions(),
        Tar.extensions(),
        Jpeg.extensions(),
        Zstd.extensions(),
        Lzma.extensions(),
    ]
    .into_iter()
    .flatten()
    .collect();

    Err(format!(
        "Unsupported file format. Use one of the following extensions: {:?}",
        supported_extensions
    ))
}
