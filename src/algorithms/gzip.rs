use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use crate::pack::Pack;

pub struct Gzip;

impl Pack for Gzip {
    fn pack(&self, files: Vec<&Path>, output: &Path) -> io::Result<()> {
        if files.len() != 1 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "GZIP compression supports only one input file"));
        }

        let input_file = files[0];
        let mut input = File::open(input_file)?;
        let output_file = File::create(output)?;
        let mut encoder = GzEncoder::new(output_file, Compression::default());

        let mut buffer = Vec::new();
        input.read_to_end(&mut buffer)?;
        encoder.write_all(&buffer)?;
        encoder.finish()?;

        Ok(())
    }

    fn unpack(&self, file: &Path, output_dir: &Path) -> io::Result<()> {
        let mut input = File::open(file)?;
        let file_name = file.file_name().unwrap().to_str().unwrap();
        let original_file_name = file_name.trim_end_matches(".gz");
        let output_path = output_dir.join(original_file_name);
        let mut output_file = File::create(output_path)?;
        let mut decoder = GzDecoder::new(&mut input);

        let mut buffer = Vec::new();
        decoder.read_to_end(&mut buffer)?;
        output_file.write_all(&buffer)?;

        Ok(())
    }

    fn extensions(&self) -> Vec<&'static str> {
        vec!["gz"]
    }
}
