use std::collections::{BTreeSet, BTreeMap};

#[cfg(test)]
mod test;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Clone)]
pub enum Token {
	/// An actual element of the domain of a structure
	Element(String),

	/// A string of text with no inherent meaning other than to be a shorcut for a more complicated expression
	/// Think of it as syntax
	Literal(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum PatternToken {
	Concrete(Token),
	Variable(String),
}

pub type Expression = Vec<Token>;

pub fn get_match_bindings<'a>(pattern: &[PatternToken], expression: &'a [Token]) -> Option<BTreeMap<String, &'a [Token]>> {
	let (pattern_token, token) = match (pattern.get(0), expression.get(0)) {
		(Some(pattern_token), Some(token)) => (pattern_token, token),
		(None, None) => return Some(BTreeMap::new()),
		_ => return None,
	};

	match pattern_token {
		PatternToken::Concrete(pattern_token) => {
			if pattern_token == token {
				return get_match_bindings(&pattern[1..], &expression[1..]);
			} else {
				None
			}
		},

		PatternToken::Variable(name) => {
			for i in 1..=expression.len() {
				if let Some(mut bindings) = get_match_bindings(&pattern[i..], &expression[i..]) {

					let binding = &expression[0..i];

					match bindings.get(name) {
						Some(existing_binding) => {
							if existing_binding == &binding {
								return Some(bindings);
							}
						},
						None => {
							bindings.insert(name.clone(), binding);
							return Some(bindings);
						},
					}
				}
			};

			None
		},
	}
}

pub struct Definition(Vec<PatternToken>,  Vec<PatternToken>);

impl Definition {
	pub fn new(a: Vec<PatternToken>, b: Vec<PatternToken>) -> Self {
		Self(a, b)
	}

	fn transform<'a>(from: &[PatternToken], to: &[PatternToken], expression: &'a [Token]) -> Option<Vec<Token>> {
		let bindings = get_match_bindings(from, expression)?;

		let mut result = Vec::new();

		for token in to {
			match token {
				PatternToken::Concrete(token) => result.push(token.clone()),
				PatternToken::Variable(name) => {
					let binding = bindings.get(name)?;
					result.extend(binding.iter().cloned());
				},
			};
		}

		Some(result)
	}

	pub fn get_transformations(&self, expression: &[Token]) -> Vec<Expression> {
		let mut output = Vec::with_capacity(2);

		if let Some(result) = Self::transform(&self.0, &self.1, expression) {
			output.push(result);
		}

		if let Some(result) = Self::transform(&self.1, &self.0, expression) {
			output.push(result);
		}

		output
	}
}

struct Strucutre {
	domain: BTreeSet<String>,
	definitions: Vec<Definition>,
}