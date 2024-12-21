use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Target {
    Stdout,
    Stderr,
    File(PathBuf),
}
