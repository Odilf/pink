mod file_resolver;
mod std_resolver;

#[cfg(feature = "embedded_std")]
mod embedded;

#[cfg(test)]
mod test;

pub use file_resolver::FileResolver;
pub use std_resolver::StdResolver;

#[cfg(feature = "embedded_std")]
pub use embedded::EmbeddedStdResolver;

/// A trait for resolving names to values.
pub trait Resolver {
    type Error: std::error::Error;

    /// Resolves a name to a value.
    fn resolve(&mut self, name: &str) -> Result<String, Self::Error>;
}
