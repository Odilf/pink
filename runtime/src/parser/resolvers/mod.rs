mod file_resolver;
mod std_resolver;

#[cfg(feature = "embedded_std")]
mod embedded;

mod chain;
mod function;
mod map;
#[cfg(test)]
mod test;

pub use chain::Chain;
pub use file_resolver::FileResolver;
pub use map::MapResolver;
pub use std_resolver::StdResolver;

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
