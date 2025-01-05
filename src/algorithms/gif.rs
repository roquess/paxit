use std::fs::File;
use std::io::{self, BufWriter};
use std::path::Path;
use image::{open, ImageFormat};
use crate::pack::Pack;

pub struct Gif;

impl Pack for Gif {
    fn pack(&self, files: Vec<&Path>, output: &Path) -> io::Result<()> {
        if files.len() != 1 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "GIF compression supports only one input file"));
        }

        let input_file = files[0];
        let img = open(input_file).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let mut output_file = BufWriter::new(File::create(output)?);
        img.write_to(&mut output_file, ImageFormat::Gif).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Ok(())
    }

    fn unpack(&self, _file: &Path, _output_dir: &Path) -> io::Result<()> {
        Err(io::Error::new(io::ErrorKind::Unsupported, "Unpacking GIF is not implemented"))
    }

    fn extensions(&self) -> Vec<&'static str> {
        vec!["gif"]
    }
}

