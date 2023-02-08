use std::collections::BTreeSet;
use once_cell::sync::Lazy;

use crate::parser::{ParseError, self};

use super::{Structure, Token};

/// The "instrinsic" structure is part of the language itself.
/// 
/// It reserves curly braces, brackets, parentheses, and commas.
pub static INTRINSIC: Lazy<Structure> = Lazy::new(|| {
	let reserved = BTreeSet::from([
		"{", "}", "[", "]", ",", "(", ")",
	].map(|s| s.to_owned()));

	Structure::create(BTreeSet::new(), reserved, Vec::new()).unwrap()
});

impl Structure {
	pub fn eval(&self, expression: &[Token]) -> Vec<Token> {
		// TODO: lol
		let mut output = expression.to_vec();
		let mut i = 0;

		loop {
			i += 1;
			let mut changed = false;
			
			for definition in &self.definitions {
				if let Some(transformation) = definition.into_preferred(&output) {
					output = transformation;
					changed = true;
				}
			}

			if !changed || i > 100 {
				break;
			}
		}

		output
	}

	pub fn eval_str(&self, input: &str) -> Result<Vec<Token>, ParseError> {
		let expression = parser::expression(input, &self.domain, &self.reserved)?;

		return Ok(self.eval(expression.as_slice()));
	}
}