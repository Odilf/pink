use std::{fmt::Display, path::PathBuf};

use super::Resolver;

/// Splits a path into name and root.
///
/// TODO: I would expect there to be a better way to do this. But I can't find it. sad
fn get_root_and_name(path: PathBuf) -> Option<(PathBuf, String)> {
    let name = path.file_name()?;
    let name = name.to_str()?;
    let name = &name[..name.len() - ".pink".len()];

    let mut root = path.clone();
    root.pop();

    Some((root, name.to_string()))
}

pub struct FileResolver {
    root: PathBuf,
}

impl FileResolver {
    /// Creates a new `FileResolver` from a full path, returns a tuple of the resolver and the relative name of the file.
    pub fn from_full_path(path: PathBuf) -> Option<(Self, String)> {
        let (root, name) = get_root_and_name(path)?;

        Some((Self { root }, name))
    }
}

impl Resolver for FileResolver {
    type Error = std::io::Error;

    fn resolve(&self, name: &str) -> Result<String, Self::Error> {
        let mut path = self.root.clone();
        path.push(name);
        path.set_extension("pink");

        std::fs::read_to_string(path)
    }
}

#[derive(Debug)]
struct FileResolverError {
    path: PathBuf,
}

impl Display for FileResolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not read file: {}", self.path.display())
    }
}

impl std::error::Error for FileResolverError {}