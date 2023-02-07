use std::{collections::BTreeSet, error::Error, fmt::Display, fs, io};

use nom::{
    bytes::complete::{tag as nom_tag, take_until as nom_take_until, take_while, take_while1},
    IResult,
};
// use regex::Regex;
use regex_macro::regex;

use crate::engine::{Definition, PatternToken, Structure, Token};

#[cfg(test)]
mod test;

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
            Err(_) => Err(ParseError::ExpectedKeyword {
                expected: tag.to_string(),
                found: input.to_string(),
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
            Err(_) => Err(ParseError::ExpectedKeyword {
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

/// Parses the *whole* input string as an expression
fn pattern<'a>(
    input: &'a str,
    domain: &BTreeSet<String>,
    reserved: &BTreeSet<String>,
) -> Vec<PatternToken> {
    let mut result = Vec::new();

    // TODO: I'm stupid this can be done in one function
    pattern_recurse(input, domain, reserved, &mut result);

    return result;
}

fn pattern_recurse<'a>(
    input: &'a str,
    domain: &BTreeSet<String>,
    reserved: &BTreeSet<String>,
    accomulator: &mut Vec<PatternToken>,
) {
    if input.len() == 0 {
        return;
    }

    let input = input.trim();

    for literal in reserved {
        if let Ok(rest) = tag(literal.as_str())(input) {
            accomulator.push(PatternToken::Concrete(Token::Literal(literal.to_string())));
            return pattern_recurse(rest, domain, reserved, accomulator);
        }
    }

    for element in domain {
        if let Ok(rest) = tag(element.as_str())(input) {
            accomulator.push(PatternToken::Concrete(Token::Element(element.to_string())));
            return pattern_recurse(rest, domain, reserved, accomulator);
        }
    }

    let result: IResult<&str, &str> = take_while1(|c: char| c.is_alphanumeric() || c == '_')(input);

    let (rest, variable) = match result {
        Ok(result) => result,
        Err(_) => (&input[1..], &input[0..1]),
    };

    accomulator.push(PatternToken::Variable(variable.to_string()));
    return pattern_recurse(rest, domain, reserved, accomulator);
}

fn definition<'a>(
    input: &'a str,
    domain: &BTreeSet<String>,
    reserved: &BTreeSet<String>,
) -> Result<(&'a str, Definition), ParseError> {
    // TODO: This currently would make things like `==` not work
    let (rest, lhs) = take_until("=")(input)?;

    // let rest = tag("=")(rest)?;
    let (rest, rhs) = take_until(";")(rest)?;
    // let rest = tag(";")(rest)?;

    let preferred = pattern(lhs, domain, reserved);
    let other = pattern(rhs, domain, reserved);

    let definition = Definition::new(preferred, other);

    return Ok((rest, definition));
}

/// TODO: Maybe make a type for inputs with comments and without. To further ensure safety.
/// Then we can add a trait and make it super generic! But that might be unecessary.
fn strip_comments(input: String) -> String {
    input.replace(regex!("#.*"), "")
}

pub fn parse(input: String) -> Result<Structure, nom::Err<nom::error::Error<String>>> {
    let input = strip_comments(input);
    let input = input.as_str();

    let (input, domain) = domain(input).unwrap();
    let (input, reserved) = reserve(input).unwrap();

    let mut definitions = Vec::new();
    let mut input = input;
    while let Ok((rest, definition)) = definition(input, &domain, &reserved) {
        input = rest;
        definitions.push(definition);
    }

    let Ok(structure) = Structure::new(domain, reserved, definitions) else {
        // wut
        // TODO: Probaly wrong
        return Err(nom::Err::Error(nom::error::Error::new(input.to_string(), nom::error::ErrorKind::Tag)))
    };

    return Ok(structure);
}

pub fn parse_file(path: &str) -> io::Result<Structure> {
    let input = fs::read_to_string(path)?;
    return Ok(parse(input).expect("pls fix lol"));
}

#[derive(Debug)]
enum ParseError {
    ExpectedKeyword { expected: String, found: String },

    // For file handling shenaningans
    Io(io::Error),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for ParseError {}
