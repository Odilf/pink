#[cfg(test)]
mod test;

use std::{collections::BTreeSet, error::Error, fmt::Display, fs, io};

use nom::{
    bytes::complete::{tag as nom_tag, take_until as nom_take_until, take_while, take_while1},
    IResult,
};
// use regex::Regex;
use regex_macro::regex;

use crate::engine::{Definition, PatternToken, Structure, Token, StructureError, eval::INTRINSIC};

/// Trims the start of the input
fn trim_start(input: &str) -> &str {
    let result: IResult<_, _> = take_while(char::is_whitespace)(input);
    match result {
        Ok((input, _)) => input,
        Err(_) => input,
    }
}

/// Nom wrapper, make it work better for current use case.
fn tag<'a>(tag: &'a str) -> impl Fn(&'a str) -> Result<&'a str, ParseError> {
    move |input: &str| {
        let result: IResult<_, _> = nom_tag(tag)(input);
        match result {
            Ok((input, _)) => Ok(input),
            Err(_) => Err(ParseError::Expected {
                expected: tag.to_string(),
                found: input[..tag.len().min(input.len())].to_string(),
            }),
        }
    }
}

/// Nom wrapper, make it work better for current use case.
/// Also consumes the tag.
fn take_until<'a>(tag: &'a str) -> impl Fn(&'a str) -> Result<(&'a str, &'a str), ParseError> {
    move |input: &str| {
        let result: IResult<_, _> = nom_take_until(tag)(input);
        match result {
            Ok((input, inside)) => {
                let input = &input[tag.len()..]; // Also consume tag
                Ok((input, inside))
            }
            Err(_) => Err(ParseError::Expected {
                expected: tag.to_string(),
                found: input.to_string(),
            }),
        }
    }
}

fn keyword_set<'a>(
    input: &'a str,
    keyword: &'a str,
) -> Result<(&'a str, BTreeSet<String>), ParseError> {
    let input = trim_start(input);
    let input = tag(keyword)(input)?;

    let input = trim_start(input);
    let input = tag("{")(input)?;

    let input = trim_start(input);
    let (input, elements) = take_until("}")(input)?;

    // let input = tag("}")(input)?;

    return Ok((
        input,
        elements.split(",").map(|s| s.trim().to_owned()).collect(),
    ));
}

fn domain(input: &str) -> Result<(&str, BTreeSet<String>), ParseError> {
    keyword_set(input, "domain")
}

fn reserve(input: &str) -> Result<(&str, BTreeSet<String>), ParseError> {
    keyword_set(input, "reserve")
}

fn uses(input: &str) -> Result<(&str, Option<&str>), ParseError> {
    let input = trim_start(input);
    match tag("uses")(input) {
        Ok(input) => {
            let input = trim_start(input);
            let (input, path) = take_until(";")(input)?;
            
            Ok((input, Some(path)))
        }
        
        Err(_) => Ok((input, None))
    }
}

/// Parses the *whole* input string as an expression
fn pattern<'a>(
    input: &'a str,
    domain: &BTreeSet<String>,
    reserved: &BTreeSet<String>,
) -> Vec<PatternToken> {
    if input.len() == 0 {
        return Vec::new();
    }

    let input = input.trim();
    
    for literal in reserved {
        if let Ok(rest) = tag(literal.as_str())(input) {
            let mut pattern = pattern(rest, domain, reserved);
            pattern.insert(0, PatternToken::Concrete(Token::Literal(literal.to_string())));
            return pattern;
        }
    }

    for element in domain {
        if let Ok(rest) = tag(element.as_str())(input) {
            let mut pattern = pattern(rest, domain, reserved);
            pattern.insert(0, PatternToken::Concrete(Token::Element(element.to_string())));
            return pattern;
        }
    }

    let result: IResult<&str, &str> = take_while1(|c: char| c.is_alphanumeric() || c == '_')(input);

    let (rest, variable) = match result {
        Ok(result) => result,
        Err(_) => (&input[1..], &input[0..1]),
    };

    let mut pattern = pattern(rest, domain, reserved);
    pattern.insert(0, PatternToken::Variable(variable.to_string()));
    return pattern;
}

/// Parses the *whole* input string as an expression
pub fn expression<'a>(
    input: &'a str,
    domain: &BTreeSet<String>,
    reserved: &BTreeSet<String>,
) -> Result<Vec<Token>, ParseError> {
    if input.len() == 0 {
        return Ok(Vec::new());
    }

    let input = input.trim();
    
    for literal in reserved {
        if let Ok(rest) = tag(literal.as_str())(input) {
            let mut expression = expression(rest, domain, reserved)?;
            expression.insert(0, Token::Literal(literal.to_string()));
            return Ok(expression);
        }
    }

    for element in domain {
        if let Ok(rest) = tag(element.as_str())(input) {
            let mut expression = expression(rest, domain, reserved)?;
            expression.insert(0, Token::Element(element.to_string()));
            return Ok(expression);
        }
    }

    return Err(ParseError::UknownToken(input.to_string()));
}

fn definition<'a>(
    input: &'a str,
    domain: &BTreeSet<String>,
    reserved: &BTreeSet<String>,
) -> Result<(&'a str, Definition), ParseError> {
    // TODO: This currently would make things like `==` not work
    let (rest, lhs) = take_until("=")(input)?;
    let (rest, rhs) = take_until(";")(rest)?;

    let other = pattern(lhs, domain, reserved);
    let preferred = pattern(rhs, domain, reserved);

    let definition = Definition::new(preferred, other);

    return Ok((rest, definition));
}

/// TODO: Maybe make a type for inputs with comments and without. To further ensure safety.
/// Then we can add a trait and make it super generic! But that might be unecessary.
fn strip_comments(input: String) -> String {
    input.replace(regex!("#.*"), "")
}

pub fn parse(input: String) -> Result<Structure, ParseError> {
    let input = strip_comments(input);
    let input = input.as_str();

    let (input, mut domain) = domain(input)?;
    let (input, mut reserved) = reserve(input)?;

    // TODO: Implement actual dependencies and a dependency graph with `INTRINSIC` as the root. 
    // This is very bad. 
    domain.append(&mut INTRINSIC.domain.clone());
    reserved.append(&mut INTRINSIC.reserved.clone());

    let mut definitions = Vec::new();
    let mut input = input;
    while let Ok((rest, definition)) = definition(input, &domain, &reserved) {
        input = rest;
        definitions.push(definition);
    }


    let structure = match Structure::create(domain, reserved, definitions)  {
        Ok(structure) => structure,
        Err(StructureError::DomainAndReservedOverlap { culprit }) => {
            return Err(ParseError::DomainAndReservedOverlap { culprit })
        }
    };

    return Ok(structure);
}

pub fn parse_file(path: &str) -> Result<Structure, ParseError> {
    let input = match fs::read_to_string(path) {
        Ok(input) => input,
        Err(e) => return Err(ParseError::Io(e)),
    };

    return Ok(parse(input)?);
}

#[derive(Debug)]
pub enum ParseError {
    Expected { expected: String, found: String },
    DomainAndReservedOverlap { culprit: String },
    UknownToken(String),

    // For file handling shenaningans
    Io(io::Error),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ParseError::Expected { expected, found } => {
                format!("Expected {}, found {}", expected, found)
            }
            ParseError::DomainAndReservedOverlap { culprit } => {
                format!("Domain and reserved overlap: {}", culprit)
            }
            ParseError::UknownToken(token) => format!("Unknown token: {}", token),
            ParseError::Io(e) => format!("IO error: {}", e),
        };

        write!(f, "{}", message)
    }
}

impl Error for ParseError {}
