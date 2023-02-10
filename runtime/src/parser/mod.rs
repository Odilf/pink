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

fn get_reserved(runtime: &PartialRuntime) -> Vec<&String> {
    runtime
        .iter()
        .filter_map(|(name, structure)| structure.as_ref().map(|s| (name, s)))
        .flat_map(|(_name, structure)| structure.get_reserved().iter())
        .collect()
}

fn get_domain(runtime: &PartialRuntime) -> Vec<&String> {
    runtime
        .iter()
        .filter_map(|(name, structure)| structure.as_ref().map(|s| (name, s)))
        .flat_map(|(_name, structure)| structure.get_domain().iter())
        .collect()
}

/// Parses the *whole* input string as an expression
fn pattern<'a>(
    input: &'a str,
    domain: &Vec<&String>,
    reserved: &Vec<&String>,
) -> Vec<PatternToken> {
    if input.is_empty() {
        return Vec::new();
    }

    let input = input.trim();

    for literal in reserved.clone() {
        if let Ok(rest) = tag(literal.as_str())(input) {
            let mut pattern = pattern(rest, domain, reserved);
            pattern.insert(
                0,
                PatternToken::Concrete(Token::Literal(literal.to_string())),
            );
            return pattern;
        }
    }

    for element in domain.clone() {
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

    let result: IResult<&str, &str> = take_while1(|c: char| c.is_alphabetic() || c == '_')(input);
    let (rest, variable) = match result {
        Ok(result) => result,
        Err(_) => (&input[1..], &input[0..1]), // Get one character if it's not alphabetic
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
    domain: &Vec<&String>,
    reserved: &Vec<&String>,
) -> Result<(&'a str, Definition), ParseError> {
    let (rest, definition) = take_until(";")(input)?;
    let definition = pattern(definition, domain, reserved);
                                                                                    // TODO: This allocates a string for no reason
    let mut sides = definition.split(|token| token == &PatternToken::Concrete(Token::Literal("=".to_string())));

    let Some(lhs) = sides.next() else {
        return Err(ParseError::Expected {
            expected: "=".to_string(),
            found: input.to_string(),
        })
    };

    let Some(rhs) = sides.next() else {
        return Err(ParseError::Expected {
            expected: "=".to_string(),
            found: input.to_string(),
        })
    };

    assert!(sides.next().is_none(), "Too many `=` in definition!");

    // TODO: This currently would make things like `==` not work
    // let (rest, lhs) = take_until("=")(input)?;
    // let (rest, rhs) = take_until(";")(rest)?;

    // let lhs = pattern(lhs, domain, reserved);
    // let rhs = pattern(rhs, domain, reserved);

    let definition = Definition::new(lhs.to_vec(), rhs.to_vec());

    Ok((rest, definition))
}

/// TODO: Maybe make a type for inputs with comments and without. To further ensure safety.
/// Then we can add a trait and make it super generic! But that might be unecessary.
/// It would be better to just rewrite the parser without nom in a nicer way tailored to the project.
fn strip_comments(input: String) -> String {
    input.replace(regex!("#.*"), "")
}

/// Splits a path into name and root.
///
/// TODO: I would expect there to be a better way to do this. But I can't find it. sad
fn get_root_and_name(path: PathBuf) -> Result<(PathBuf, String), ParseError> {
    let name = path.file_name().unwrap();
    let name = name.to_str().unwrap();
    let name = &name[..name.len() - ".pink".len()];

    let mut root = path.clone();
    root.pop();

    Ok((root, name.to_string()))
}

// TODO: Should generalize parsing to having a function that given a name returns a string, not being specific to the filesystem.
// Would be useful for porting to web and using as a library, in general
pub fn parse_file(path: PathBuf) -> Result<Runtime, ParseError> {
    // Add intrinsic to runtime by default
    // TODO: Maybe this should be in it's own "setup" function or smth
    let mut runtime = BTreeMap::from([("intrinsic".to_string(), Some(Structure::intrinsic()))]);

    let (root, name) = get_root_and_name(path)?;

    parse_file_into_runtime(&root, &name, &mut runtime)?;

    let runtime = runtime
        .into_iter()
        .filter_map(|(name, structure)| structure.map(|structure| (name, structure)))
        .collect();

    Ok(Runtime::new(runtime))
}

// The `Option` is `None` if the file has not been parsed yet. This is used to prevent circular dependencies.
type PartialRuntime = BTreeMap<String, Option<Structure>>;

/// Parses a file into a partial runtime
fn parse_file_into_runtime(
    root: &Path,
    name: &str,
    runtime: &mut PartialRuntime,
) -> Result<(), ParseError> {
    let file_path = root.join(format!("{name}.pink"));
    let input = match fs::read_to_string(&file_path) {
        Ok(input) => input,

        // TODO: Maybe make this a `ParseError::FileNotFound` or something
        Err(err) => {
            println!("Path {:?} doesn't exists, apparently", file_path);
            return Err(ParseError::Io(err))
        },
    };

    let input = strip_comments(input);
    let input = input.as_str();

    let (input, domain) = domain(input)?;
    let (input, reserved) = reserve(input)?;
    let (input, dependencies) = parse_use(input)?;

    for dependency in dependencies {
        // Check for circular dependencies
        match runtime.get(&dependency) {
            // Already parsed
            Some(Some(_)) => continue,

            // Currently parsing
            Some(None) => {
                return Err(ParseError::CircularDependency {
                    // TODO: Actually find cycles properly
                    cycle: vec![dependency.clone(), name.to_string()],
                });
            }

            // Not parsed yet
            None => (),
        }

        runtime.insert(dependency.clone(), None);

        parse_file_into_runtime(root, &dependency, runtime)?;
    }

    // Get definitions
    // TODO: Check here for collisions between domain and reserved
    let mut full_domain = get_domain(runtime);
    full_domain.extend(domain.iter());

    let mut full_reserved = get_reserved(runtime);
    full_reserved.extend(reserved.iter());

    let mut definitions = Vec::new();
    let mut input = input;
    while let Ok((rest, definition)) = definition(input, &full_domain, &full_reserved) {
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
    CircularDependency { cycle: Vec<String> },
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
            ParseError::CircularDependency { cycle } => {
                format!("Found circular dependency: {}", cycle.join(" -> "))
            }
            ParseError::UknownToken(token) => format!("Unknown token in: \"{}\"", token),
            ParseError::Io(e) => format!("IO error: {}", e),
        };

        write!(f, "{}", message)
    }
}

impl Error for ParseError {}
