use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt::Display,
};

#[cfg(test)]
mod test;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Token<'a> {
    /// An actual element of the domain of a structure.
    Element(&'a str),

    /// A string of text with no inherent meaning other than to be a shorcut for a more complicated expression.
    /// Think of it as syntax.
    Literal(&'a str),
}

#[derive(Debug, PartialEq, Eq)]
pub enum PatternToken<'a> {
    Concrete(Token<'a>),
    Variable(&'a str),
}

pub type Expression<'a> = Vec<Token<'a>>;

pub fn get_match_bindings<'a>(
    pattern: &'a [PatternToken],
    expression: &'a [Token],
) -> Option<BTreeMap<&'a str, &'a [Token<'a>]>> {
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
        }

        PatternToken::Variable(name) => {
            for i in 0..=expression.len() {
                if let Some(mut bindings) = get_match_bindings(&pattern[1..], &expression[i..]) {
                    let binding = &expression[0..i];

                    match bindings.get(name) {
                        Some(existing_binding) => {
                            if existing_binding == &binding {
                                return Some(bindings);
                            }
                        }
                        None => {
                            bindings.insert(name, binding);
                            return Some(bindings);
                        }
                    }
                }
            }

            None
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Definition<'a> {
    preferred: Vec<PatternToken<'a>>,
    other: Vec<PatternToken<'a>>,
}

impl<'a> Definition<'a> {
    pub fn new(preferred: Vec<PatternToken<'a>>, other: Vec<PatternToken<'a>>) -> Self {
        Self { preferred, other }
    }

    fn transform(
        from: &'a [PatternToken],
        to: &'a [PatternToken],
        expression: &'a [Token],
    ) -> Option<Vec<Token<'a>>> {
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

    pub fn get_transformations(&'a self, expression: &'a [Token]) -> Vec<Expression> {
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
pub struct Structure<'a> {
    pub domain: BTreeSet<&'a str>,
    pub reserved: BTreeSet<&'a str>,
    pub definitions: Vec<Definition<'a>>,
}

impl<'a> Structure<'a> {
    pub fn new(
        domain: BTreeSet<&'a str>,
        reserved: BTreeSet<&'a str>,
        definitions: Vec<Definition<'a>>,
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

    pub fn get_transformations(&self, expression: &'a [Token]) -> Vec<Expression> {
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
