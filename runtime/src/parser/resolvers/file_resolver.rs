use std::{fmt::Display, path::PathBuf};

use super::{Resolver, StdResolver};

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

/// A resolver based on the filesystem.
///
/// Note: It uses the `StdResolver` to resolve standard library modules
/// before attempting to look into files.
pub struct FileResolver {
    cwd: PathBuf,
}

impl FileResolver {
    // TODO: This shouldn't exist.
    // pub fn from_full_path(path: PathBuf) -> Option<(Self, String)> {
    //     let (root, name) = get_root_and_name(path)?;

    //     Some((Self { root }, name))
    // }

    pub fn new() -> Self {
        Self {
            cwd: PathBuf::from(""),
        }
    }
}

impl Resolver for FileResolver {
    type Error = std::io::Error;

    fn resolve(&mut self, name: &str) -> Result<String, Self::Error> {
        // Try to resolve the module using the standard library resolver
        if let Ok(module) = StdResolver::resolve(name) {
            return Ok(module);
        }

        let mut path = self.cwd.clone();

        path.push(name);
        path.set_extension("pink");

        let (root, _name) = get_root_and_name(path.clone()).unwrap();

        self.cwd = root;

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
