use std::fs::File;
use std::io::{self, copy};
use std::path::Path;
use zip::write::ZipWriter;
use zip::read::ZipArchive;
use zip::write::FileOptions;
use crate::pack::Pack;

pub struct Zip;

impl Pack for Zip {
    fn pack(&self, files: Vec<&Path>, output: &Path) -> io::Result<()> {
        let file = File::create(output)?;
        let mut zip = ZipWriter::new(file);
        let options: FileOptions<()> = Default::default();

        for file_path in files {
            let file_name = file_path.file_name().unwrap().to_str().unwrap();
            zip.start_file(file_name, options)?;
            let mut f = File::open(file_path)?;
            copy(&mut f, &mut zip)?;
        }

        zip.finish()?;
        Ok(())
    }

    fn unpack(&self, file: &Path, output_dir: &Path) -> io::Result<()> {
        let f = File::open(file)?;
        let mut archive = ZipArchive::new(f)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = output_dir.join(file.name());
            if file.name().ends_with('/') {
                std::fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(p)?;
                    }
                }
                let mut outfile = File::create(&outpath)?;
                copy(&mut file, &mut outfile)?;
            }

            // Set the permissions
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Some(mode) = file.unix_mode() {
                    std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode))?;
                }
            }
        }

        Ok(())
    }

    fn extensions(&self) -> Vec<&'static str> {
        vec!["zip"]
    }
}

