use std::collections::BTreeSet;

use super::{Expression, Runtime};

impl Runtime {
    fn get_lower_neighbours(
        &self,
        expression: &Expression,
    ) -> Vec<Expression> {
        let mut neighbours = Vec::new();

        for size in 1..=expression.tokens.len() {
            for (window_start, window) in expression.tokens.windows(size).enumerate() {
                for definition in self.definitions() {
                    if let Some(lowered_window) = definition.lower(window) {
                        let mut lowered = expression.tokens[..window_start].to_vec();
                        lowered.extend(lowered_window.tokens);
                        lowered.extend(expression.tokens[window_start + size..].to_vec());

                        let neighbour = Expression::new(lowered);
                        neighbours.push(neighbour);

                        break;
                    }
                }
            }
        }

        neighbours
    }

    /// Returns the lowest possible evaluation
    pub fn eval(
        &self,
        expression: Expression,
        callback: &mut impl FnMut(&BTreeSet<Expression>),
    ) -> Expression {
        self.evaluations(expression, callback)
            .into_iter()
            .next()
            .expect("Should have at least the original expression")
    }

    /// Returns all possible evaluations and runs the callback on each iteration
    pub fn evaluations(
        &self,
        expression: Expression,
        callback: &mut impl FnMut(&BTreeSet<Expression>),
    ) -> BTreeSet<Expression> {
        let mut visited = BTreeSet::new();
        let mut queue = Vec::new();

        queue.push(expression);

        while let Some(expression) = queue.pop() {
            if visited.contains(&expression) {
                continue;
            }

            visited.insert(expression.clone());
            
            let neighbours = self.get_lower_neighbours(&expression);

            queue.extend(neighbours);

            callback(&visited);
        }

        visited
    }
}
