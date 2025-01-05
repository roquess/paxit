pub mod bmp;
pub mod dds;
pub mod farbfeld;
pub mod gif;
pub mod hdr;
pub mod ico;
pub mod jpeg;
pub mod lzma;
pub mod png;
pub mod pnm;
pub mod tar;
pub mod tga;
pub mod tiff;
pub mod webp;
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
        Box::new(dds::Dds) as Box<dyn Pack>,
        Box::new(farbfeld::Farbfeld) as Box<dyn Pack>,
        Box::new(gif::Gif) as Box<dyn Pack>,
        Box::new(hdr::Hdr) as Box<dyn Pack>,
        Box::new(ico::Ico) as Box<dyn Pack>,
        Box::new(jpeg::Jpeg) as Box<dyn Pack>,
        Box::new(lzma::Lzma) as Box<dyn Pack>,
        Box::new(png::Png) as Box<dyn Pack>,
        Box::new(pnm::Pnm) as Box<dyn Pack>,
        Box::new(tar::Tar) as Box<dyn Pack>,
        Box::new(tga::Tga) as Box<dyn Pack>,
        Box::new(tiff::Tiff) as Box<dyn Pack>,
        Box::new(webp::Webp) as Box<dyn Pack>,
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
        dds::Dds.extensions(),
        farbfeld::Farbfeld.extensions(),
        gif::Gif.extensions(),
        hdr::Hdr.extensions(),
        ico::Ico.extensions(),
        jpeg::Jpeg.extensions(),
        lzma::Lzma.extensions(),
        png::Png.extensions(),
        pnm::Pnm.extensions(),
        tar::Tar.extensions(),
        tga::Tga.extensions(),
        tiff::Tiff.extensions(),
        webp::Webp.extensions(),
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
