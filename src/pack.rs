use std::io;
use std::path::Path;

/// The `Pack` trait defines the methods required for a compression algorithm.
///
/// Implementors of this trait must provide methods for packing (compressing) and unpacking (decompressing) files,
/// as well as a method to return the file extensions supported by the compression algorithm.
pub trait Pack {
    /// Compresses the given files into a single output file.
    ///
    /// # Arguments
    ///
    /// * `files` - A vector of file paths to be compressed.
    /// * `output` - The path to the output file where the compressed data will be stored.
    ///
    /// # Returns
    ///
    /// An `io::Result<()>` indicating success or failure.
    fn pack(&self, files: Vec<&Path>, output: &Path) -> io::Result<()>;

    /// Decompresses the given file into the specified output directory.
    ///
    /// # Arguments
    ///
    /// * `file` - The path to the file to be decompressed.
    /// * `output_dir` - The path to the directory where the decompressed files will be stored.
    ///
    /// # Returns
    ///
    /// An `io::Result<()>` indicating success or failure.
    fn unpack(&self, file: &Path, output_dir: &Path) -> io::Result<()>;

    /// Returns a vector of file extensions supported by the compression algorithm.
    ///
    /// # Returns
    ///
    /// A vector of static string slices representing the supported file extensions.
    fn extensions(&self) -> Vec<&'static str>;
}

