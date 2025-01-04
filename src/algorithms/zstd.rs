use std::fs::File;
use std::io;
use std::path::Path;
use zstd::stream::{Decoder, Encoder};
use crate::pack::Pack;

pub struct Zstd;

impl Pack for Zstd {
    fn pack(&self, files: Vec<&Path>, output: &Path) -> io::Result<()> {
        if files.len() != 1 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Zstd compression supports only one input file"));
        }

        let input_file = files[0];
        let mut input = File::open(input_file)?;
        let output_file = File::create(output)?;
        let mut encoder = Encoder::new(output_file, 3)?; // Compression level 3
        io::copy(&mut input, &mut encoder)?;
        encoder.finish()?;

        Ok(())
    }

    fn unpack(&self, file: &Path, output_dir: &Path) -> io::Result<()> {
        let input = File::open(file)?;
        let output_path = output_dir.join(file.file_name().unwrap());
        let mut output_file = File::create(output_path)?;
        let mut decoder = Decoder::new(input)?;
        io::copy(&mut decoder, &mut output_file)?;

        Ok(())
    }

    fn extensions(&self) -> Vec<&'static str> {
        vec!["zst"]
    }
}


