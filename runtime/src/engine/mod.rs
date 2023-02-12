pub mod eval;

#[cfg(test)]
mod test;

use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt::Display,
};

use crate::{
    matching::get_match_bindings,
    parser::{self, ParseError},
};

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
        use termion::{
            color::{Fg, LightMagenta},
            style::Reset,
        };
        match self {
            Token::Element(element) => write!(f, "{}{element}{}", Fg(LightMagenta), Reset)?,

            // TODO: I'm afraid something could be wrong here in different terminals
            Token::Literal(literal) => write!(f, "{literal}")?,
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

impl Display for PatternToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use termion::style::{Bold, Italic, Reset};

        match self {
            Self::Concrete(token) => write!(f, "{token}"),
            Self::Variable(name) => write!(f, "{}{name}{}", Bold, Reset),
            Self::SpreadVariable(name) => write!(f, "{}{}{name}{}", Bold, Italic, Reset),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Expression {
    pub tokens: Vec<Token>,
}

impl Expression {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }
}

impl PartialOrd for Expression {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.tokens.len().partial_cmp(&other.tokens.len())? {
            std::cmp::Ordering::Equal => self.tokens.partial_cmp(&other.tokens),
            ordering => ordering.into(),
        }
    }
}

impl Ord for Expression {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.tokens.len().cmp(&other.tokens.len()) {
            std::cmp::Ordering::Equal => self.tokens.cmp(&other.tokens),
            ordering => ordering,
        }
    }
}

impl From<&[Token]> for Expression {
    fn from(tokens: &[Token]) -> Self {
        Self {
            tokens: tokens.to_vec(),
        }
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
/// A definition has a "high" and a "low" side. Pink tries to lower the definitions.
///
/// It is defined as `high => low`, so expressions will generally be moved to the right.
pub struct Definition {
    low: Vec<PatternToken>,
    high: Vec<PatternToken>,
}

impl Definition {
    /// Definitions are defined as `high => low` (so `rhs` is `low` and `lhs` is `high`)
    pub fn new(lhs: Vec<PatternToken>, rhs: Vec<PatternToken>) -> Self {
        Self {
            high: lhs,
            low: rhs,
        }
    }

    /// Transform an expression from one pattern to another.
    fn transform<'a>(
        from: &[PatternToken],
        to: &[PatternToken],
        expression: &'a [Token],
    ) -> Option<Expression> {
        let (single_bindings, spread_bindings) = get_match_bindings(from, expression)?;

        let mut result = Vec::new();

        for token in to {
            match token {
                PatternToken::Concrete(token) => result.push(token.clone()),
                PatternToken::Variable(name) => {
                    let binding = *single_bindings.get(name)?;
                    result.push(binding.clone());
                }
                PatternToken::SpreadVariable(name) => {
                    let binding = spread_bindings.get(name)?;
                    result.extend_from_slice(binding);
                }
            };
        }

        Some(Expression::new(result))
    }

    pub fn lower(&self, expression: &[Token]) -> Option<Expression> {
        Self::transform(&self.high, &self.low, expression)
    }

    pub fn raise(&self, expression: &[Token]) -> Option<Expression> {
        Self::transform(&self.low, &self.high, expression)
    }
}

impl Display for Definition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in &self.high {
            write!(f, "{} ", token)?;
        }

        write!(f, "=> ")?;

        for token in &self.low {
            write!(f, "{} ", token)?;
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Structure {
    domain: BTreeSet<String>,
    reserved: BTreeSet<String>,
    definitions: Vec<Definition>,
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
            return Err(StructureError::DomainAndReservedOverlap {
                culprit: culprit.to_owned(),
            });
        }

        Ok(Self {
            domain,
            reserved,
            definitions,
        })
    }

    /// The "intrinsic" structure is defined by the language itself
    ///
    /// It reserves curly braces, parentheses, and commas.
    pub fn intrinsic() -> Self {
        let reserved = BTreeSet::from(["{", "}", ",", "(", ")", "="].map(|s| s.to_owned()));

        Structure {
            domain: BTreeSet::new(),
            reserved,
            definitions: Vec::new(),
        }
    }

    /// Creates an empty structure
    pub fn empty() -> Self {
        Structure {
            domain: BTreeSet::new(),
            reserved: BTreeSet::new(),
            definitions: Vec::new(),
        }
    }

    pub fn get_reserved(&self) -> &BTreeSet<String> {
        &self.reserved
    }

    pub fn get_domain(&self) -> &BTreeSet<String> {
        &self.domain
    }
}

impl Display for Structure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let domain: Vec<_> = self.domain.iter().cloned().collect();
        writeln!(f, "Domain: {{ {} }}", domain.join(", "))?;

        let reserved: Vec<_> = self.reserved.iter().cloned().collect();
        writeln!(f, "Reserved: {{ {} }}", reserved.join(", "))?;

        writeln!(f, "Definitions: ")?;

        for definition in &self.definitions {
            writeln!(f, "- {definition};")?;
        }

        Ok(())
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
                write!(
                    f,
                    "Domain and reserved keywords overlap (\"{}\" appears in both)",
                    culprit
                )
            }
        }
    }
}

impl Error for StructureError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Runtime {
    pub structures: BTreeMap<String, Structure>,
}

impl Runtime {
    pub fn new(structures: BTreeMap<String, Structure>) -> Self {
        Self { structures }
    }

    /// Iterator that goes through each element in the domains of the structures of the runtime
    pub fn domain(&self) -> impl Iterator<Item = &String> + '_ {
        self.structures
            .iter()
            .flat_map(|(_name, structure)| structure.domain.iter())
    }

    /// Iterator that goes through each literal in the reserved keywords of the structures of the runtime
    pub fn reserved(&self) -> impl Iterator<Item = &String> + '_ {
        self.structures
            .iter()
            .flat_map(|(_name, structure)| structure.reserved.iter())
    }

    /// Iterator that goes through each literal in the reserved keywords of the structures of the runtime
    pub fn definitions(&self) -> impl Iterator<Item = &Definition> + '_ {
        self.structures
            .iter()
            .flat_map(|(_name, structure)| structure.definitions.iter())
    }

    pub fn contains(&self, name: &str) -> bool {
        self.structures.contains_key(name)
    }

    pub fn insert(&mut self, name: String, structure: Structure) -> Option<Structure> {
        self.structures.insert(name, structure)
    }

    pub fn from_partial(structures: &BTreeMap<String, Option<Structure>>) -> Self {
        let mut runtime = Runtime::new(BTreeMap::new());

        for (name, structure) in structures {
            if let Some(structure) = structure {
                runtime.insert(name.to_owned(), structure.to_owned());
            }
        }

        runtime
    }

    pub fn parse_expression(&self, expression: &str) -> Result<Expression, ParseError> {
        parser::expression(expression, self)
    }
}

impl Display for Runtime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (name, structure) in &self.structures {
            writeln!(f, "{name}:")?;
            writeln!(f, "{structure}")?;
        }

        Ok(())
    }
}
