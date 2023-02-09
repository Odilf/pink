#[cfg(test)]
mod test;

use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt::Display,
    fs, io,
    path::{Path, PathBuf},
};

use nom::{
    bytes::complete::{tag as nom_tag, take_until as nom_take_until, take_while, take_while1},
    IResult,
};

// use regex::Regex;
use regex_macro::regex;

use crate::engine::{
    Definition, Expression, PatternToken, Runtime, Structure, StructureError, Token,
};

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

    return Ok((
        input,
        elements
            .split(',')
            .map(|s| s.trim().to_owned())
            .filter(|s| !s.is_empty())
            .collect(),
    ));
}

fn domain(input: &str) -> Result<(&str, BTreeSet<String>), ParseError> {
    keyword_set(input, "domain")
}

fn reserve(input: &str) -> Result<(&str, BTreeSet<String>), ParseError> {
    keyword_set(input, "reserve")
}

// TODO: Make `use` optional
fn parse_use(input: &str) -> Result<(&str, BTreeSet<String>), ParseError> {
    keyword_set(input, "use")
}

/// Parses the *whole* input string as an expression
fn pattern<'a>(
    input: &'a str,
    domain: &BTreeSet<String>,
    reserved: &BTreeSet<String>,
) -> Vec<PatternToken> {
    if input.is_empty() {
        return Vec::new();
    }

    let input = input.trim();

    for literal in reserved {
        if let Ok(rest) = tag(literal.as_str())(input) {
            let mut pattern = pattern(rest, domain, reserved);
            pattern.insert(
                0,
                PatternToken::Concrete(Token::Literal(literal.to_string())),
            );
            return pattern;
        }
    }

    for element in domain {
        if let Ok(rest) = tag(element.as_str())(input) {
            let mut pattern = pattern(rest, domain, reserved);
            pattern.insert(
                0,
                PatternToken::Concrete(Token::Element(element.to_string())),
            );
            return pattern;
        }
    }

    // TODO: Implement spread variables

    let result: IResult<&str, &str> = take_while1(|c: char| c.is_alphanumeric() || c == '_')(input);
    let (rest, variable) = match result {
        Ok(result) => result,
        Err(_) => (&input[1..], &input[0..1]),
    };

    let mut pattern = pattern(rest, domain, reserved);
    pattern.insert(0, PatternToken::Variable(variable.to_string()));
    pattern
}

/// Parses the *whole* input string as an expression
pub fn expression(input: &str, runtime: &Runtime) -> Result<Expression, ParseError> {
    if input.is_empty() {
        return Ok(Expression::new(Vec::new()));
    }

    let input = input.trim();

    for literal in runtime.reserved() {
        if let Ok(rest) = tag(literal.as_str())(input) {
            let mut expression = expression(rest, runtime)?;
            expression
                .tokens
                .insert(0, Token::Literal(literal.to_string()));
            return Ok(expression);
        }
    }

    for element in runtime.domain() {
        if let Ok(rest) = tag(element.as_str())(input) {
            let mut expression = expression(rest, runtime)?;
            expression
                .tokens
                .insert(0, Token::Element(element.to_string()));
            return Ok(expression);
        }
    }

    Err(ParseError::UknownToken(input.to_string()))
}

fn definition<'a>(
    input: &'a str,
    domain: &BTreeSet<String>,
    reserved: &BTreeSet<String>,
) -> Result<(&'a str, Definition), ParseError> {
    // TODO: This currently would make things like `==` not work
    let (rest, lhs) = take_until("=")(input)?;
    let (rest, rhs) = take_until(";")(rest)?;

    let lhs = pattern(lhs, domain, reserved);
    let rhs = pattern(rhs, domain, reserved);

    let definition = Definition::new(lhs, rhs);

    Ok((rest, definition))
}

/// TODO: Maybe make a type for inputs with comments and without. To further ensure safety.
/// Then we can add a trait and make it super generic! But that might be unecessary.
fn strip_comments(input: String) -> String {
    input.replace(regex!("#.*"), "")
}

fn get_name_and_root(path: PathBuf) -> Result<(String, PathBuf), ParseError> {
    let name = path.file_name().unwrap();
    let name = name.to_str().unwrap();
    let name = &name[..name.len() - ".pink".len()];

    let mut rest = path.clone();
    rest.pop();

    Ok((name.to_string(), rest))
}

pub fn parse_file(path: PathBuf) -> Result<Runtime, ParseError> {
    let mut runtime = BTreeMap::new();

    let (name, path) = get_name_and_root(path)?;

    parse_file_into_runtime(&path, &name, &mut runtime)?;

    let runtime = runtime
        .into_iter()
        .filter_map(|(name, structure)| structure.map(|structure| (name, structure)))
        .collect();

    Ok(Runtime::new(runtime))
}

/// Parses a file into a map `String -> Option<Structure>`. The `Option` is `None` if the file
/// has not been parsed yet. This is used to prevent circular dependencies.
fn parse_file_into_runtime(
    root: &Path,
    name: &str,
    runtime: &mut BTreeMap<String, Option<Structure>>,
) -> Result<(), ParseError> {
    let input = match fs::read_to_string(root.join(format!("{name}.pink"))) {
        Ok(input) => input,

        // TODO: Maybe make this a `ParseError::FileNotFound` or something
        Err(err) => return Err(ParseError::Io(err)),
    };

    let input = strip_comments(input);
    let input = input.as_str();

    let (input, domain) = domain(input)?;
    let (input, reserved) = reserve(input)?;
    let (input, dependencies) = parse_use(input)?;

    for dependency in dependencies {
        if runtime.contains_key(&dependency) {
            continue;
        }

        runtime.insert(dependency.clone(), None);

        parse_file_into_runtime(root, &dependency, runtime)?;
    }

    // Get definitions
    let mut definitions = Vec::new();
    let mut input = input;
    while let Ok((rest, definition)) = definition(input, &domain, &reserved) {
        input = rest;
        definitions.push(definition);
    }

    let structure = match Structure::create(domain, reserved, definitions) {
        Ok(structure) => structure,
        Err(StructureError::DomainAndReservedOverlap { culprit }) => {
            return Err(ParseError::DomainAndReservedOverlap { culprit })
        }
    };

    runtime.insert(name.to_string(), Some(structure));

    Ok(())
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
