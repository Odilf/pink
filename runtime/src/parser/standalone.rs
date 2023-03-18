//! Module for standalone parsers

use std::collections::BTreeSet;

use nom::{
    bytes::complete::{tag as nom_tag, take_until as nom_take_until, take_while, take_while1},
    IResult,
};

use crate::{
    engine::{Definition, Expression, PatternToken, Token},
    Runtime,
};

use super::{ParseError, PartialRuntime};

/// Crate level documentation?

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

pub fn domain(input: &str) -> Result<(&str, BTreeSet<String>), ParseError> {
    keyword_set(input, "domain")
}

pub fn reserve(input: &str) -> Result<(&str, BTreeSet<String>), ParseError> {
    keyword_set(input, "reserve")
}

// TODO: Make `use` optional
pub fn parse_use(input: &str) -> Result<(&str, BTreeSet<String>), ParseError> {
    keyword_set(input, "use")
}

pub fn get_reserved(runtime: &PartialRuntime) -> Vec<&String> {
    runtime
        .iter()
        .filter_map(|(name, structure)| structure.as_ref().map(|s| (name, s)))
        .flat_map(|(_name, structure)| structure.get_reserved().iter())
        .collect()
}

pub fn get_domain(runtime: &PartialRuntime) -> Vec<&String> {
    runtime
        .iter()
        .filter_map(|(name, structure)| structure.as_ref().map(|s| (name, s)))
        .flat_map(|(_name, structure)| structure.get_domain().iter())
        .collect()
}

/// Parses the *whole* input string as an expression
pub fn pattern(input: &str, domain: &Vec<&String>, reserved: &Vec<&String>) -> Vec<PatternToken> {
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

    // Normal variable
    let result: IResult<_, _> = take_while1(|c: char| c.is_alphanumeric() || c == '_')(input);
    let (rest, variable) = match result {
        Ok(result) => result,
        Err(_) => (&input[1..], &input[0..1]), // Get one character if it's not alphabetic
    };

    // Spread variables
    // TODO: Uggo
    let result: IResult<_, _> = nom_tag("...")(rest);
    match result {
        Ok((rest, _)) => {
            let mut pattern = pattern(rest, domain, reserved);
            pattern.insert(0, PatternToken::SpreadVariable(variable.to_string()));
            pattern
        }
        Err(_) => {
            let mut pattern = pattern(rest, domain, reserved);
            pattern.insert(0, PatternToken::Variable(variable.to_string()));
            pattern
        }
    }
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

pub fn definition<'a>(
    input: &'a str,
    domain: &Vec<&String>,
    reserved: &Vec<&String>,
) -> Result<(&'a str, Vec<Definition>), ParseError> {
    let (rest, definition) = take_until(";")(input)?;

    let mut sides = definition.split("=>");

    let Some(lhs) = sides.next() else {
        return Err(ParseError::Expected {
            expected: "=> or <=>".to_string(),
            found: rest.to_string(),
        })
    };

    let Some(rhs) = sides.next() else {
        return Err(ParseError::Expected {
            expected: "something after => or <=>".to_string(),
            found: rest.to_string(),
        })
    };

    // TODO: Just... allow this lol
    assert!(
        sides.next().is_none(),
        "Too many `=>` or `<=>` in definition"
    );

    let double = lhs.ends_with('<');

    let lhs = if double { &lhs[..lhs.len() - 1] } else { lhs };

    let lhs = pattern(lhs, domain, reserved);
    let rhs = pattern(rhs, domain, reserved);

    let mut result = Vec::new();

    result.push(Definition::new(lhs.to_vec(), rhs.to_vec()));

    if double {
        result.push(Definition::new(rhs, lhs));
    }

    Ok((rest, result))
}
