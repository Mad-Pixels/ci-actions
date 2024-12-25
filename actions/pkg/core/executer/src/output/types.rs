use std::path::PathBuf;

/// Represents the target where log messages can be directed.
#[derive(Debug, Clone)]
pub enum Target {
    /// Standard output stream.
    Stdout,
    /// Standard error stream.
    Stderr,
    /// A file specified by a path.
    File(PathBuf),
}

impl Target {
    /// Creates a new `Target` for standard output.
    ///
    /// # Example
    ///
    /// ```rust
    /// use executer::Target;
    ///
    /// let target = Target::stdout();
    /// ```
    pub fn stdout() -> Self {
        Target::Stdout
    }

    /// Creates a new `Target` for standard error.
    ///
    /// # Example
    ///
    /// ```rust
    /// use executer::Target;
    ///
    /// let target = Target::stderr();
    /// ```
    pub fn stderr() -> Self {
        Target::Stderr
    }

    /// Creates a new `Target` for a file.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file where logs will be written.
    ///
    /// # Example
    ///
    /// ```rust
    /// use executer::Target;
    /// use std::path::PathBuf;
    ///
    /// let path = PathBuf::from("log.txt");
    /// let target = Target::file(path);
    /// ```
    pub fn file<P: Into<PathBuf>>(path: P) -> Self {
        Target::File(path.into())
    }
}
