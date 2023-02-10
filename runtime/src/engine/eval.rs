use std::collections::{BTreeSet, VecDeque};

use super::{Expression, Runtime};

pub enum LowerResult {
    Lowered(Expression),
    Unchanged(Expression),
}

impl Runtime {
    fn get_lower_neighbours(&self, expression: Expression) -> BTreeSet<Expression> {
        let mut neighbours = BTreeSet::new();

        for size in 1..=expression.tokens.len() {
            for (window_start, window) in expression.tokens.windows(size).enumerate() {
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

    pub fn eval(&self, expression: Expression) -> Expression {
        self.evaluations(expression).iter().next().expect("Should have at least the original expression").clone()
    }

    /// Returns a set of all possible evaluations
    pub fn evaluations(&self, expression: Expression) -> BTreeSet<Expression> {
        let mut visited = BTreeSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(expression);

        while !queue.is_empty() {
            let expression = queue.pop_front().unwrap();

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
