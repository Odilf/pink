pub mod resolvers;
mod standalone;

#[cfg(test)]
mod test;

pub use standalone::expression;

use std::{collections::BTreeMap, error::Error, fmt::Display, io, path::PathBuf};

use regex_macro::regex;

use crate::engine::{Runtime, Structure, StructureError};

use self::{
    resolvers::{FileResolver, Resolver, StdResolver},
    standalone::{definition, domain, get_domain, get_reserved, parse_use, reserve},
};

/// TODO: Maybe make a type for inputs with comments and without. To further ensure safety.
/// Then we can add a trait and make it super generic! But that might be unecessary.
/// It would be better to just rewrite the parser without nom in a nicer way tailored to the project.
fn strip_comments(input: String) -> String {
    let regex = regex!("#.*");
    regex.replace_all(&input, "").to_string()
}

pub fn parse<R: Resolver>(name: &str, resolver: &mut R) -> Result<Runtime, ParseError> {
    let mut partial_runtime =
        BTreeMap::from([("intrinsic".to_string(), Some(Structure::intrinsic()))]);

    let input = resolver.resolve(name).unwrap();

    parse_into_runtime(&input, name, resolver, &mut partial_runtime)?;

    let runtime = partial_runtime
        .into_iter()
        // This shouldn't *really* be done this way. The `Option`s are unnecessary.
        .filter_map(|(name, structure)| structure.map(|structure| (name, structure)))
        .collect();

    Ok(Runtime::new(runtime))
}

// The `Option` is `None` if the file has not been parsed yet. This is used to prevent circular dependencies.
type PartialRuntime = BTreeMap<String, Option<Structure>>;

fn parse_into_runtime<R: Resolver>(
    input: &str,
    name: &str,
    resolver: &mut R,
    runtime: &mut PartialRuntime,
) -> Result<(), ParseError> {
    let input = strip_comments(input.to_string());

    let (input, domain) = domain(&input)?;
    let (input, reserved) = reserve(input)?;
    let (input, dependencies) = parse_use(input)?;

    for dependency in dependencies {
        match runtime.get(&dependency) {
            // Already parsed
            Some(Some(_)) => continue,

            // Circular dependency
            Some(None) => {
                return Err(ParseError::CircularDependency {
                    cycle: vec![dependency.clone(), name.to_string()],
                })
            }

            // Not parsed yet
            None => (),
        }

        runtime.insert(dependency.clone(), None);

        let dependecy_program = resolver
            .resolve(&dependency)
            .expect("Hande failures of resolver (basically not finding files)");

        match parse_into_runtime(&dependecy_program, &dependency, resolver, runtime) {
            Ok(()) => (),

            // Bubble up circular dependency error
            Err(ParseError::CircularDependency { mut cycle }) => {
                cycle.push(dependency);
                return Err(ParseError::CircularDependency { cycle });
            }

            Err(err) => return Err(err),
        };
    }

    let full_domain = domain.iter().chain(get_domain(runtime)).collect();
    let full_reserved = reserved.iter().chain(get_reserved(runtime)).collect();

    let mut definitions = Vec::new();
    let mut input = input;

    while let Ok((rest, mut parsed_definitions)) = definition(input, &full_domain, &full_reserved) {
        input = rest;
        definitions.append(&mut parsed_definitions);
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

/// Parse using a file resolver.
///
/// Basically, this is a convenience function for `parse` that uses a `FileResolver` to resolve names.
/// So you can just pass a path to a file and it will parse it, but also do `std/whatever`.
pub fn parse_file(path: PathBuf) -> Result<Runtime, ParseError> {
    let mut resolver = StdResolver::default().chain(FileResolver::new());

    parse(path.to_str().unwrap(), &mut resolver)
}

#[derive(Debug)]
pub enum ParseError {
    Expected { expected: String, found: String },
    DomainAndReservedOverlap { culprit: String },
    CircularDependency { cycle: Vec<String> },
    UknownToken(String),
    FileNotFound(String),

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
            ParseError::FileNotFound(file) => format!("File not found: {}", file),
        };

        write!(f, "{}", message)
    }
}

impl Error for ParseError {}
