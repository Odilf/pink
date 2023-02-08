pub mod eval;

#[cfg(test)]
mod test;

use std::{collections::BTreeSet, error::Error, fmt::Display};

use crate::matching::get_match_bindings;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Token {
    /// An actual element of the domain of a structure
    Element(String),

    /// A string of text with no inherent meaning other than to be a shorcut for a more complicated expression
    /// Think of it as syntax
    Literal(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use termion::color;
        match self {
            Token::Element(element) => write!(f, "{element}")?,

            // TODO: I'm afraid something could be wrong here in different terminals
            Token::Literal(literal) => write!(f, "{}{literal}{}", color::Fg(color::LightCyan), color::Fg(color::LightWhite))?,
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PatternToken {
    /// Either element or literal
    Concrete(Token),

    /// Variable that binds to only one token
    Variable(String),

    /// Variable that binds arbitrary number of tokens
    SpreadVariable(String),
}

pub struct Expression {
    pub tokens: Vec<Token>,
}

impl Expression {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }
}

impl From<&[Token]> for Expression {
    fn from(tokens: &[Token]) -> Self {
        Self { tokens: tokens.to_vec() }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in &self.tokens {
            write!(f, "{} ", token)?;
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Definition {
    preferred: Vec<PatternToken>,
    other: Vec<PatternToken>,
}

impl Definition {
    /// Definitions are defined as `other = preferred` (so `rhs` is `preferred` and `lhs` is `other`)
    pub fn new(lhs: Vec<PatternToken>, rhs: Vec<PatternToken>) -> Self {
        Self { preferred: rhs, other: lhs }
    }

    /// Transform an expression from one pattern to another.
    fn transform<'a>(
        from: &[PatternToken],
        to: &[PatternToken],
        expression: &'a [Token],
    ) -> Option<Expression> {
        let (mut single_bindings, mut spread_bindings) = get_match_bindings(from, expression)?;

        let mut result = Vec::new();

        for token in to {
            match token {
                PatternToken::Concrete(token) => result.push(token.clone()),
                PatternToken::Variable(name) => {
                    let binding = single_bindings.remove(name)?;
                    result.push(binding.clone());
                },
                PatternToken::SpreadVariable(name) => {
                    let binding = spread_bindings.remove(name)?;
                    result.extend_from_slice(binding);
                }
            };
        }

        Some(Expression::new(result))
    }

    pub fn into_preferred(&self, expression: &[Token]) -> Option<Expression> {
        Self::transform(&self.other, &self.preferred, expression)
    }

    pub fn out_of_preferred(&self, expression: &[Token]) -> Option<Expression> {
        Self::transform(&self.preferred, &self.other, expression)
    }

    // pub fn get_transformations(&self, expression: &[Token]) -> Vec<Expression> {
    //     let mut output = Vec::with_capacity(2);

    //     if let Some(result) = self.into_preferred(expression) {
    //         output.push(result);
    //     }

    //     if let Some(result) = self.out_of_preferred(expression) {
    //         output.push(result);
    //     }

    //     output
    // }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Structure {
    pub domain: BTreeSet<String>,
    pub reserved: BTreeSet<String>,
    pub definitions: Vec<Definition>,
}

impl Structure {
    /// Create a new structure.
    /// 
    /// It is not called `new` because it can fail and using `new` with `Result` is confusing.
    pub fn create(
        domain: BTreeSet<String>,
        reserved: BTreeSet<String>,
        definitions: Vec<Definition>,
    ) -> Result<Self, StructureError> {
        if let Some(culprit) = domain.iter().find(|d| reserved.contains(*d)) {
            return Err(StructureError::DomainAndReservedOverlap { culprit: culprit.to_owned() });
        }

        Ok(Self {
            domain,
            reserved,
            definitions,
        })
    }

    pub fn include(mut self, other: Structure) -> Result<Self, StructureError> {
        self.domain.extend(other.domain);
        self.reserved.extend(other.reserved);

        if let Some(culprit) = self.domain.iter().find(|d| self.reserved.contains(*d)) {
            // TODO: Return proper errors for this. Maybe make structures have names?
            return Err(StructureError::DomainAndReservedOverlap { culprit: culprit.to_owned() });
        }

        self.definitions.extend(other.definitions);

        return Ok(self);
    }
}


// I don't know to what extent this error is necessary. Maybe replace it with `Option`?
#[derive(Debug, PartialEq, Eq)]
pub enum StructureError {
    DomainAndReservedOverlap { culprit: String },
}

impl Display for StructureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StructureError::DomainAndReservedOverlap { culprit } => {
                write!(f, "Domain and reserved keywords overlap (\"{}\" appears in both)", culprit)
            }
        }
    }
}

impl Error for StructureError {}
