use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt::Display,
};

use crate::matching::get_match_bindings;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
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

#[derive(Debug, PartialEq, Eq)]
pub struct Definition {
    preferred: Vec<PatternToken>,
    other: Vec<PatternToken>,
}

impl Definition {
    pub fn new(preferred: Vec<PatternToken>, other: Vec<PatternToken>) -> Self {
        Self { preferred, other }
    }

    fn transform<'a>(
        from: &[PatternToken],
        to: &[PatternToken],
        expression: &'a [Token],
    ) -> Option<Vec<Token>> {
        let bindings = get_match_bindings(from, expression)?;

        let mut result = Vec::new();

        for token in to {
            match token {
                PatternToken::Concrete(token) => result.push(token.clone()),
                PatternToken::Variable(name) => {
                    let binding = bindings.get(name)?;
                    result.extend(binding.iter().cloned());
                }
            };
        }

        Some(result)
    }

    pub fn get_transformations(&self, expression: &[Token]) -> Vec<Expression> {
        let mut output = Vec::with_capacity(2);

        if let Some(result) = Self::transform(&self.preferred, &self.other, expression) {
            output.push(result);
        }

        if let Some(result) = Self::transform(&self.other, &self.preferred, expression) {
            output.push(result);
        }

        output
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Structure {
    pub domain: BTreeSet<String>,
    pub reserved: BTreeSet<String>,
    pub definitions: Vec<Definition>,
}

impl Structure {
    pub fn new(
        domain: BTreeSet<String>,
        reserved: BTreeSet<String>,
        definitions: Vec<Definition>,
    ) -> Result<Self, StructureCreationError> {
        if domain.iter().any(|d| reserved.contains(d)) {
            return Err(StructureCreationError::DomainAndReservedOverlap);
        }

        Ok(Self {
            domain,
            reserved,
            definitions,
        })
    }

    pub fn get_transformations(&self, expression: &[Token]) -> Vec<Expression> {
        let mut output = Vec::new();

        for definition in &self.definitions {
            output.extend(definition.get_transformations(expression));
        }

        output
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum StructureCreationError {
    DomainAndReservedOverlap,
}

impl Display for StructureCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StructureCreationError::DomainAndReservedOverlap => {
                write!(f, "Domain and reserved keywords overlap")
            }
        }
    }
}

impl Error for StructureCreationError {}
