#[cfg(test)]
mod test;

use std::collections::BTreeMap;

use crate::engine::{PatternToken, Token};

pub fn get_match_bindings<'a, 'b>(
    pattern: &'a [PatternToken],
    expression: &'b [Token],
) -> Option<BTreeMap<&'a String, &'b [Token]>> {
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
