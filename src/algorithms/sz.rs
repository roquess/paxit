use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use crate::pack::Pack;
use snap::write::FrameEncoder;
use snap::read::FrameDecoder;

pub struct Sz;

impl Pack for Sz {
    fn pack(&self, files: Vec<&Path>, output: &Path) -> io::Result<()> {
        if files.len() != 1 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Snappy compression supports only one input file"));
        }

        let input_file = files[0];
        let mut input = File::open(input_file)?;
        let output_file = File::create(output)?;
        let mut encoder = FrameEncoder::new(output_file);

        let mut buffer = Vec::new();
        input.read_to_end(&mut buffer)?;
        encoder.write_all(&buffer)?;

        Ok(())
    }

    fn unpack(&self, file: &Path, output_dir: &Path) -> io::Result<()> {
        let mut input = File::open(file)?;
        let file_name = file.file_name().unwrap().to_str().unwrap();
        let original_file_name = file_name.trim_end_matches(".sz");
        let output_path = output_dir.join(original_file_name);
        let mut output_file = File::create(output_path)?;
        let mut decoder = FrameDecoder::new(&mut input);

        let mut buffer = Vec::new();
        decoder.read_to_end(&mut buffer)?;
        output_file.write_all(&buffer)?;

        Ok(())
    }

    fn extensions(&self) -> Vec<&'static str> {
        vec!["sz"]
    }
}
