use std::collections::BTreeSet;
use once_cell::sync::Lazy;

use crate::parser::{ParseError, self};

use super::{Structure, Expression};

/// The "instrinsic" structure is part of the language itself.
/// 
/// It reserves curly braces, parentheses, and commas.
pub static INTRINSIC: Lazy<Structure> = Lazy::new(|| {
	let reserved = BTreeSet::from([
		"{", "}", ",", "(", ")",
	].map(|s| s.to_owned()));

	Structure::create(BTreeSet::new(), reserved, Vec::new()).unwrap()
});

pub enum LowerResult {
	Lowered(Expression),
	Unchanged(Expression),
}

impl Structure {
	pub fn lower(&self, expression: Expression) -> LowerResult {


		for definition in &self.definitions {
			if let Some(lower) = definition.lower(expression.tokens.as_slice()) {
				println!("Matched definition: {definition}\n");
				return LowerResult::Lowered(lower);
			}
		}

		println!("Didn't find a definition that matches, expression is lowered all the way down.");
		LowerResult::Unchanged(expression)
	}

	pub fn eval(&self, expression: Expression) -> Expression {
		println!("lowering expression: {expression}");
		match self.lower(expression) {
			LowerResult::Lowered(lowered) => self.eval(lowered),
			LowerResult::Unchanged(expression) => expression,
		}
	}

	pub fn eval_str(&self, input: &str) -> Result<Expression, ParseError> {
		let expression = parser::expression(input, &self.domain, &self.reserved)?;

		return Ok(self.eval(expression));
	}
}