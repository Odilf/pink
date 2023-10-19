use thiserror::Error;

use super::Resolver;

struct EmbeddedProgram {
	name: &'static str,
	program: &'static str,
}

macro_rules! get_program {
	($name:tt) => {
		EmbeddedProgram {
			name: $name,
			program: include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../standard_library/", $name, ".pink")),
		}
	};
}

/// A resolver that embeds all the standard library files the binary itself
/// 
/// Warning: It might considerably increas binary size
pub struct EmbeddedStdResolver {}

const PROGRAMS: [EmbeddedProgram; 6] = [
	get_program!("binary"),
	get_program!("core"),
	get_program!("function calls"),
	get_program!("peano"),
	get_program!("propositional logic"),
	get_program!("sets"),
];

impl Resolver for EmbeddedStdResolver {
	type Error = EmbeddedResolverError;

	fn resolve(&mut self, name: &str) -> Result<String, Self::Error> {
		let program = PROGRAMS.iter().find(|p| p.name == name).ok_or(EmbeddedResolverError::NotFound(name.into()))?;
		Ok(program.program.to_string())
	}
}

#[derive(Debug, Error)]
pub enum EmbeddedResolverError {
	#[error("File {0} is not embedded into the program")]
	NotFound(String),
}
