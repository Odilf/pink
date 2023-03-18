mod file_resolver;

pub use file_resolver::FileResolver;

/// A trait for resolving names to values.
pub trait Resolver {
    type Error: std::error::Error;

    /// Resolves a name to a value.
    fn resolve(&self, name: &str) -> Result<String, Self::Error>;
}
