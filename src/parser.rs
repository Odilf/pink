pub trait Parsable {
	fn parse(input: &str) -> Result<Self, ParseError> where Self: Sized;
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct ParseError {
	falty_input: String,
	explanation: String
}

impl ParseError {
	pub fn new(input: &str, explanation: &str) -> Self {
		ParseError { 
			falty_input: input.to_string(),
			explanation: explanation.to_string(),
		}
	}
}