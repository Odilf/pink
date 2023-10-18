mod file_resolver;
mod std_resolver;

#[cfg(test)]
mod test;

pub use file_resolver::FileResolver;
pub use std_resolver::StdResolver;

/// A trait for resolving names to values.
pub trait Resolver {
    type Error: std::error::Error;

    /// Resolves a name to a value.
    fn resolve(&mut self, name: &str) -> Result<String, Self::Error>;
}
