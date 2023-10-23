mod file_resolver;
mod std_resolver;

#[cfg(feature = "embedded_std")]
mod embedded;

#[cfg(test)]
mod test;
mod function;
mod map;
mod chain;

pub use file_resolver::FileResolver;
pub use std_resolver::StdResolver;
pub use chain::Chain;
pub use map::MapResolver;

#[cfg(feature = "embedded_std")]
pub use embedded::EmbeddedStdResolver;

/// A trait for resolving names to values.
pub trait Resolver {
    type Error: std::error::Error;

    /// Resolves a name to a value.
    fn resolve(&mut self, name: &str) -> Result<String, Self::Error>;

    fn chain<R: Resolver>(self, resolver: R) -> chain::Chain<Self, R>
    where
        Self: Sized,
    {
        chain::Chain::new(self, resolver)
    }
}
