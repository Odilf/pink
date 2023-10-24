use std::collections::BTreeSet;

use wasm_bindgen::prelude::*;

use crate::parser::parse;
use crate::resolvers::{Chain, EmbeddedStdResolver, MapResolver, Resolver};
use crate::{Expression, Runtime};

#[wasm_bindgen]
pub struct AsyncRuntime {
    runtime: Option<Runtime>,
    resolver: Chain<EmbeddedStdResolver, MapResolver>,
}

#[wasm_bindgen]
impl AsyncRuntime {
    pub fn new() -> AsyncRuntime {
        Self {
            runtime: None,
            resolver: EmbeddedStdResolver::default().chain(MapResolver::new()),
        }
    }

    pub fn send_program(&mut self, name: String, program: String) {
        self.resolver.resolver2.insert(name, program)
    }

    pub fn parse_with_main(&mut self, name: &str) {
        self.runtime = parse(name, &mut self.resolver).ok();
    }

    pub async fn evaluations(
        &mut self,
        expression: String,
        callback: js_sys::Function,
        on_finish: js_sys::Function,
        performance: &web_sys::Performance,
    ) -> JsValue {
        let expression_result = self.runtime.as_mut().unwrap().parse_expression(&expression);

        let expression = match expression_result {
            Ok(expression) => expression,
            Err(error) => {
                return error.to_string().into();
            }
        };

        let runtime = self.runtime.as_ref().unwrap();

        runtime.evaluations(
            expression,
            &mut wasm_loop_callback(callback, performance.clone()),
        );

        on_finish.call0(&JsValue::NULL).unwrap();

        JsValue::UNDEFINED
    }
}

fn wasm_loop_callback(
    callback: js_sys::Function,
    performance: web_sys::Performance,
) -> impl FnMut(&BTreeSet<Expression>) {
    let time_start = performance.now();

    let mut smallest_expression: Option<Expression> = None;

    move |evaluations: &BTreeSet<Expression>| {
        if smallest_expression.is_none() {
            smallest_expression = Some(evaluations.iter().next().unwrap().clone());
            callback
                .call2(
                    &JsValue::NULL,
                    &JsValue::from(smallest_expression.as_ref().unwrap().to_string()),
                    &JsValue::from(0.0),
                )
                .unwrap();
        }

        let candidate = evaluations.iter().next().unwrap();

        if candidate < &smallest_expression.clone().unwrap() {
            smallest_expression = Some(candidate.clone());

            let time_elapsed = (performance.now() - time_start) / 1000.0;
            callback
                .call2(
                    &JsValue::NULL,
                    &candidate.to_string().into(),
                    &time_elapsed.into(),
                )
                .unwrap();
        }
    }
}
