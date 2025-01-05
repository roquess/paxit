pub mod bmp;
pub mod gif;
pub mod jpeg;
pub mod lzma;
pub mod png;
pub mod tar;
pub mod tiff;
pub mod zip;
pub mod zstd;

use super::pack::Pack;
use std::path::Path;

pub fn determine_compressor(mode: &str, files: &[&Path]) -> Result<Box<dyn Pack>, String> {
    let extension = match mode {
        "c" | "compress" | "p" | "pack" => files[0].extension().and_then(|ext| ext.to_str()),
        "u" | "uncompress" | "unpack" => files[0].extension().and_then(|ext| ext.to_str()),
        _ => None,
    };

    let compressors = vec![
        Box::new(bmp::Bmp) as Box<dyn Pack>,
        Box::new(gif::Gif) as Box<dyn Pack>,
        Box::new(jpeg::Jpeg) as Box<dyn Pack>,
        Box::new(lzma::Lzma) as Box<dyn Pack>,
        Box::new(png::Png) as Box<dyn Pack>,
        Box::new(tar::Tar) as Box<dyn Pack>,
        Box::new(tiff::Tiff) as Box<dyn Pack>,
        Box::new(zip::Zip) as Box<dyn Pack>,
        Box::new(zstd::Zstd) as Box<dyn Pack>,
    ];

    for compressor in compressors {
        if compressor.extensions().contains(&extension.unwrap_or("")) {
            return Ok(compressor);
        }
    }

    let supported_extensions: Vec<&str> = vec![
        bmp::Bmp.extensions(),
        gif::Gif.extensions(),
        jpeg::Jpeg.extensions(),
        lzma::Lzma.extensions(),
        png::Png.extensions(),
        tar::Tar.extensions(),
        tiff::Tiff.extensions(),
        zip::Zip.extensions(),
        zstd::Zstd.extensions(),
    ]
    .into_iter()
    .flatten()
    .collect();

    Err(format!(
        "Unsupported file format. Use one of the following extensions: {:?}",
        supported_extensions
    ))
}
