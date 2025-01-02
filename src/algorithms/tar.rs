use std::fs::File;
use std::io;
use std::path::Path;
use tar::Archive;
use tar::Builder;
use crate::pack::Pack;

pub struct Tar;

impl Pack for Tar {
    fn pack(&self, files: Vec<&Path>, output: &Path) -> io::Result<()> {
        let file = File::create(output)?;
        let mut archive = Builder::new(file);

        for file_path in files {
            let file_name = file_path.file_name().unwrap().to_str().unwrap();
            archive.append_path_with_name(file_path, file_name)?;
        }

        archive.finish()?;
        Ok(())
    }

    fn unpack(&self, file: &Path, output_dir: &Path) -> io::Result<()> {
        let f = File::open(file)?;
        let mut archive = Archive::new(f);

        archive.unpack(output_dir)?;
        Ok(())
    }
}

