#[cfg(test)]
mod test;

use std::collections::BTreeMap;

use crate::engine::{PatternToken, Token};

pub fn get_match_bindings<'a, 'b>(
    pattern: &'a [PatternToken],
    expression: &'b [Token],
) -> Option<BTreeMap<&'a String, &'b [Token]>> {
    let mut bindings = BTreeMap::new();

    match_bindings_recurse(pattern, expression, &mut bindings)?;

    return Some(bindings);
}

fn match_bindings_recurse<'a, 'b>(
    pattern: &'a [PatternToken],
    expression: &'b [Token],
    bindings: &mut BTreeMap<&'a String, &'b [Token]>,
) -> Option<()> {
    let (pattern_token, token) = match (pattern.get(0), expression.get(0)) {
        (Some(pattern_token), Some(token)) => (pattern_token, token),
        (None, None) => return Some(()),
        _ => return None,
    };

    match pattern_token {
        PatternToken::Concrete(pattern_token) => {
            if pattern_token == token {
                return match_bindings_recurse(&pattern[1..], &expression[1..], bindings);
            } else {
                None
            }
        }

        PatternToken::Variable(name) => {
            for i in 1..=expression.len() {
                let binding = &expression[0..i];

                match bindings.get(name) {
                    Some(existing_binding) => {
                        if existing_binding == &binding {
                            if match_bindings_recurse(&pattern[1..], &expression[i..], bindings).is_some() {
                                return Some(())
                            };
                        };
                    }
                    None => {
                        // TODO: It is kind of ugly to remove and add a binding each time but I'm not sure it can be done in a better way.
                        // It kind of seems this is the reason the pure recursive approach didn't work in the first place.
                        // The fact that without this the only test that failed is the one that failed with the recursive approach,
                        // this makes me think that this is the reason.
                        bindings.insert(name, binding);
                        if match_bindings_recurse(&pattern[1..], &expression[i..], bindings).is_some() {
                            return Some(());
                        }
                        bindings.remove(name);
                    }
                }
            }

            None
        }
    }
}