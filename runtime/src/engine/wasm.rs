use std::collections::{BTreeMap, BTreeSet};
use std::hint::black_box;
use std::time::{Duration, Instant};

use futures::{Stream, SinkExt, Future};
use futures::channel::mpsc::{self, Receiver, Sender};
use wasm_bindgen::prelude::*;

use crate::parser::parse;
use crate::{Runtime, Structure, Expression};
use crate::resolvers::{Chain, EmbeddedStdResolver, MapResolver, Resolver};

#[wasm_bindgen]
struct AsyncRuntime {
	runtime: Option<Runtime>,
	resolver: Chain<EmbeddedStdResolver, MapResolver>,
	sender: mpsc::Sender<Result<JsValue, JsValue>>,
	stream: Option<web_sys::ReadableStream>,
}

#[wasm_bindgen]
impl AsyncRuntime {
	pub fn new() -> AsyncRuntime {
		const BUFFER_SIZE: usize = 10;
		let (tx, rx) = mpsc::channel(BUFFER_SIZE);

		let wasm_stream = wasm_streams::ReadableStream::from_stream(rx);
		let wasm_stream = wasm_stream.as_raw().clone().unchecked_into();

		Self {
			runtime: None,
			resolver: EmbeddedStdResolver::default().chain(MapResolver::new()),
			sender: tx,
			stream: Some(wasm_stream),
		}
	}

	pub fn get_stream(&mut self) -> Option<web_sys::ReadableStream> {
		self.stream.take()
	}

	pub fn send_program(&mut self, name: String, program: String) {
		self.resolver.resolver2.insert(name, program)
	}

	pub fn set_main_program(&mut self, name: &str) {
		self.runtime = parse(name, &mut self.resolver).ok();
	}

	pub async fn evaluate(&mut self, expression: String) {
		let expression = self.runtime.as_mut().unwrap().parse_expression(&expression).map_err(|e| e.to_string()).unwrap();

		// black_box(expression);

		// let Some(runtime) = self.runtime.as_mut() else {
		// 	// self.sender.send(Ok("FAIL".into())).await.unwrap();
		// 	return;
		// };

		// runtime.evaluations(expression, &mut wasm_loop_callback(&mut self.sender));
		// let result = expression.evaluate(&self.runtime, &mut self.resolver).await.map_err(|e| e.to_string())?;
		// Ok(result.into())
		self.sender.send(Ok("TODO".into())).await.unwrap();

		let mut count = 0;
		for i in 0..1000 {
			count += i ^ 0x6578;
		}

		self.sender.send(Ok(count.into())).await.unwrap();
	}
}

fn wasm_loop_callback(sender: &mut Sender<Result<JsValue, JsValue>>) -> impl FnMut(&BTreeSet<Expression>) + '_ {
    let time_start = Instant::now();

    let mut smallest_expression: Option<Expression> = None;

    move |evaluations: &BTreeSet<Expression>| {
        if smallest_expression.is_none() {
            smallest_expression = Some(evaluations.iter().next().unwrap().clone());
        }

        let candidate = evaluations.iter().next().unwrap();

        if candidate < &smallest_expression.clone().unwrap() {
            smallest_expression = Some(candidate.clone());

			// let future = sender.feed(Ok(candidate.tokens.len().into()));
			// futures::executor::block_on(future).unwrap();
            // println!(
            //     "Found smaller expression: {} (took {:.2}s)",
            //     candidate,
            //     (Instant::now() - time_start).as_secs_f32()
            // );
        }
    }
}
