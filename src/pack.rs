use std::path::Path;
use std::io;

pub trait Pack {
    fn pack(&self, files: Vec<&Path>, output: &Path) -> io::Result<()>;
    fn unpack(&self, file: &Path, output_dir: &Path) -> io::Result<()>;
    fn extensions(&self) -> Vec<&'static str>;
}

