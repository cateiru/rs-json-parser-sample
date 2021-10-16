use std::{fs, fs::File as FsFile, io, path::Path};

pub struct File<'a> {
    pub path: &'a Path,
}

impl<'a> File<'a> {
    //! File operation class.
    //!
    //! ## Feature
    //!
    //! - Read all target file.
    //! - Read line target file.
    //!
    //! ## Example
    //!
    //! ```rust
    //! use json_parser::File;
    //! use std::{
    //!     error::Error,
    //!     path::Path,
    //! };
    //!
    //! fn main() -> Result<(), Box<dyn Error>> {
    //!     let file_op = File::new(Path::new("path/to"));
    //!
    //!     if let Some(file_op) = file_op {
    //!
    //!     }
    //!     Ok(())
    //! }
    //! ```

    /// Create instance of File class.
    ///
    /// ## Arguments
    ///
    /// - `path` - target file path. Must be file not dir.
    ///
    /// ## Returns
    ///
    /// Instance. Returns `None` if path of arguments not file path.
    pub fn new(path: &'a Path) -> Option<Self> {
        match path.exists() && path.is_file() {
            true => Some(Self { path: path }),
            _ => None,
        }
    }

    /// Read all file. used by std::fs::read_to_string
    ///
    /// ## Returns
    ///
    /// file data.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use json_parser::File;
    /// use std::{error::Error, path::Path};
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let file_op = File::new(Path::new("path/to"));
    ///
    ///     if let Some(file_op) = file_op {
    ///         let f = file_op.read_all()?;
    ///         print!("{}", f);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn read_all(&self) -> io::Result<String> {
        fs::read_to_string(self.path)
    }

    /// Read online. used by std::fs::File::open
    ///
    /// ## Returns
    ///
    /// file data.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use json_parser::File;
    /// use std::{
    ///     error::Error,
    ///     io::{BufRead, BufReader},
    ///     path::Path,
    /// };
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let file_op = File::new(Path::new("path/to"));
    ///
    ///     if let Some(file_op) = file_op {
    ///         for result in BufReader::new(file_op.read_line()?).lines() {
    ///             let l = result?;
    ///             println!("{}", l);
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn read_line(&self) -> io::Result<FsFile> {
        FsFile::open(self.path)
    }
}
