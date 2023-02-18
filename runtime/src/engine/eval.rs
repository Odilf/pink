use std::collections::{BTreeSet, VecDeque};

use super::{Expression, Runtime};

impl Runtime {
    fn get_lower_neighbours(&self, expression: Expression) -> BTreeSet<Expression> {
        let mut neighbours = BTreeSet::new();

        for size in 1..=expression.tokens.len() {
            for (window_start, window) in expression.tokens.windows(size).enumerate() {
                // let starts_with_paren = window.starts_with(&[Token::Literal("(".to_owned())]);
                // let ends_with_paren = window.ends_with(&[Token::Literal(")".to_owned())]);

                // // Count the number of open and closed parentheses
                // let open_parens = window
                //     .iter()
                //     .filter(|token| token == &&Token::Literal("(".to_owned()))
                //     .count();

                // let closed_parens = window
                //     .iter()
                //     .filter(|token| token == &&Token::Literal(")".to_owned()))
                //     .count();

                // // if starts_with_paren && ends_with_paren {
                // if starts_with_paren && ends_with_paren && open_parens == closed_parens {
                //     let lowered_window =
                //         self.eval(Expression::new(window[1..window.len() - 1].to_vec()));
                //     let mut lowered = expression.tokens[..window_start].to_vec();
                //     lowered.extend(lowered_window.tokens);
                //     lowered.extend(expression.tokens[window_start + size..].to_vec());

                //     neighbours.insert(Expression::new(lowered));
                //     continue;
                // }

                for definition in self.definitions() {
                    if let Some(lowered_window) = definition.lower(window) {
                        let mut lowered = expression.tokens[..window_start].to_vec();
                        lowered.extend(lowered_window.tokens);
                        lowered.extend(expression.tokens[window_start + size..].to_vec());

                        neighbours.insert(Expression::new(lowered));
                        break;
                    }
                }
            }
        }

        neighbours
    }

    /// Returns the lowest possible evaluation
    pub fn eval(&self, expression: Expression) -> Expression {
        self.evaluations(expression)
            .iter()
            .next()
            .expect("Should have at least the original expression")
            .clone()
    }

    /// Returns a set of all possible evaluations
    pub fn evaluations(&self, expression: Expression) -> BTreeSet<Expression> {
        let mut visited = BTreeSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(expression);

        while !queue.is_empty() {
            let expression = queue.pop_front().unwrap();

            // println!("Visiting: {}", expression);

            if visited.contains(&expression) {
                continue;
            }

            visited.insert(expression.clone());

            for neighbour in self.get_lower_neighbours(expression) {
                queue.push_back(neighbour);
            }
        }

        visited
    }
}
