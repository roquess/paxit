use std::env;
use std::path::Path;
use std::io;
use paxit::algorithms::determine_compressor;

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

