use super::Resolver;

pub struct Chain<R1: Resolver, R2: Resolver> {
	pub resolver1: R1,
	pub resolver2: R2,
}

impl<R1: Resolver, R2: Resolver> Chain<R1, R2> {
	pub fn new(resolver1: R1, resolver2: R2) -> Self {
		Self {
			resolver1,
			resolver2,
		}
	}
}

impl<R1: Resolver, R2: Resolver> Resolver for Chain<R1, R2> {
	type Error = R2::Error;

	fn resolve(&mut self, name: &str) -> Result<String, Self::Error> {
		self.resolver1
			.resolve(name)
			.or_else(|_| self.resolver2.resolve(name))
	}
}
