#[cfg(test)]
mod test;

use std::collections::BTreeMap;

use crate::engine::{PatternToken, Token};

type Bindings<'a, 'b> = (
    BTreeMap<&'a String, &'b Token>,   // Single bindings
    BTreeMap<&'a String, &'b [Token]>, // Spread bindings
);

pub fn get_match_bindings<'a, 'b>(
    pattern: &'a [PatternToken],
    expression: &'b [Token],
) -> Option<Bindings<'a, 'b>> {
    let mut single_bindings = BTreeMap::new();
    let mut spread_bindings = BTreeMap::new();

    match_bindings_recurse(
        pattern,
        expression,
        &mut single_bindings,
        &mut spread_bindings,
    )?;

    Some((single_bindings, spread_bindings))
}

fn match_bindings_recurse<'a, 'b>(
    pattern: &'a [PatternToken],
    expression: &'b [Token],
    single_bindings: &mut BTreeMap<&'a String, &'b Token>,
    spread_bindings: &mut BTreeMap<&'a String, &'b [Token]>,
) -> Option<()> {
    let (pattern_token, token) = match (pattern.get(0), expression.get(0)) {
        (Some(pattern_token), Some(token)) => (pattern_token, token),
        (None, None) => return Some(()),
        _ => return None,
    };

    match pattern_token {
        PatternToken::Concrete(pattern_token) => {
            if pattern_token == token {
                match_bindings_recurse(
                    &pattern[1..],
                    &expression[1..],
                    single_bindings,
                    spread_bindings,
                )
            } else {
                None
            }
        }

        PatternToken::Variable(variable) => match single_bindings.get(variable) {
            Some(existing_binding) => {
                if existing_binding == &token {
                    match_bindings_recurse(
                        &pattern[1..],
                        &expression[1..],
                        single_bindings,
                        spread_bindings,
                    )
                } else {
                    None
                }
            }
            None => {
                single_bindings.insert(variable, token);
                match_bindings_recurse(
                    &pattern[1..],
                    &expression[1..],
                    single_bindings,
                    spread_bindings,
                )
            }
        },

        PatternToken::SpreadVariable(variable) => {
            for i in 1..=expression.len() {
                let binding = &expression[0..i];

                match spread_bindings.get(variable) {
                    Some(existing_binding) => {
                        if existing_binding == &binding
                            && match_bindings_recurse(
                                &pattern[1..],
                                &expression[i..],
                                single_bindings,
                                spread_bindings,
                            )
                            .is_some()
                        {
                            return Some(());
                        };
                    }
                    None => {
                        // TODO: It is kind of ugly to remove and add a binding each time but I'm not sure it can be done in a better way.
                        // It kind of seems this is the reason the pure recursive approach didn't work in the first place.
                        // The fact that without this the only test that failed is the one that failed with the recursive approach,
                        // this makes me think that this is the reason.
                        spread_bindings.insert(variable, binding);

                        if match_bindings_recurse(
                            &pattern[1..],
                            &expression[i..],
                            single_bindings,
                            spread_bindings,
                        )
                        .is_some()
                        {
                            return Some(());
                        }

                        spread_bindings.remove(variable);
                    }
                }
            }

            None
        }
    }
}
