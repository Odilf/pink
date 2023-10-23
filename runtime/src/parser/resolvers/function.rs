use std::error::Error;

use super::Resolver;

pub struct FunctionResolver<F: Fn(&str) -> Result<String, Err>, Err: Error> {
    function: F,
}

impl<F: Fn(&str) -> Result<String, Err>, Err: Error> FunctionResolver<F, Err> {
	pub fn new(function: F) -> Self {
		Self { function }
	}
}

impl<F: Fn(&str) -> Result<String, Err>, Err: Error> Resolver for FunctionResolver<F, Err> {
	type Error = Err;

	fn resolve(&mut self, name: &str) -> Result<String, Self::Error> {
		(self.function)(name)
	}
}
