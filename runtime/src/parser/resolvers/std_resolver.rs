use std::{ffi::OsString, fmt::Display, path::PathBuf};

use super::Resolver;

use include_dir::{include_dir, Dir};

/// A resolver for getting the standard library modules.
///
/// Standard library modules are prefixed with `std/`.
///
/// # Example
///
/// ```
/// use pink_runtime::resolvers::{StdResolver, Resolver};
///
/// let mut resolver = StdResolver::default();
/// let module = resolver.resolve("std/peano").unwrap();
/// ```
#[derive(Default)]
pub struct StdResolver {}

impl Resolver for StdResolver {
    type Error = StdResolverError;

    fn resolve(&mut self, name: &str) -> Result<String, Self::Error> {
        let path = PathBuf::from(name);
        let mut path_iter = path.iter();

        let prefix = path_iter.next();
        if prefix != Some("std".as_ref()) {
            return Err(StdResolverError::NotValidPrefix(
                prefix.map(|s| s.to_os_string()),
            ));
        }

        let rest: PathBuf = path_iter.collect();
        let rest = rest.with_extension("pink");

        let Some(file) = STANDARD_LIBRARY.get_file(&rest) else {
            return Err(StdResolverError::NotAnStdModule(
                rest.to_str().unwrap().to_string(),
            ));
        };

        Ok(file.contents_utf8().unwrap().to_string())
    }
}

impl StdResolver {
    pub fn resolve(name: &str) -> Result<String, StdResolverError> {
        Self::default().resolve(name)
    }
}

// I don't know if it's the best idea to embed the entire standard library in the binary
const STANDARD_LIBRARY: Dir = include_dir!("$CARGO_MANIFEST_DIR/../standard_library");

#[derive(Debug)]
pub enum StdResolverError {
    NotValidPrefix(Option<OsString>),
    NotAnStdModule(String),
}

impl std::error::Error for StdResolverError {}

impl Display for StdResolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not read file")
    }
}
