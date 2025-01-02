use std::env;
use std::path::Path;
use paxit::algorithms::zip::Zip;
use paxit::algorithms::tar::Tar;
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

    let compressor: Box<dyn Pack> = match mode.as_str() {
        "c" | "compress" | "p" | "pack" => {
            if files.len() < 2 {
                eprintln!("Usage: paxit <mode> <output> <files...>");
                return Ok(());
            }
            let output = files[0];
            if output.extension().map_or(false, |ext| ext == "zip") {
                Box::new(Zip)
            } else if output.extension().map_or(false, |ext| ext == "tar") {
                Box::new(Tar)
            } else {
                eprintln!("Unsupported file format. Use .zip or .tar");
                return Ok(());
            }
        }
        "u" | "uncompress" | "unpack" => {
            if files.len() != 2 {
                eprintln!("Usage: paxit <mode> <file> <output_dir>");
                return Ok(());
            }
            let file = files[0];
            if file.extension().map_or(false, |ext| ext == "zip") {
                Box::new(Zip)
            } else if file.extension().map_or(false, |ext| ext == "tar") {
                Box::new(Tar)
            } else {
                eprintln!("Unsupported file format. Use .zip or .tar");
                return Ok(());
            }
        }
        _ => {
            eprintln!("Invalid mode. Use 'c', 'compress', 'p', 'pack' for compression and 'u', 'uncompress', 'unpack' for decompression.");
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

