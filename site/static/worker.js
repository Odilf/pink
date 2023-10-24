// ";

importScripts('/wasm-runtime/pink_runtime.js');

const { AsyncRuntime } = wasm_bindgen;

async function init_wasm() {

	await wasm_bindgen('/wasm-runtime/pink_runtime_bg.wasm');

	const runtime = AsyncRuntime.new();

	let expression = "";

	self.postMessage({ type: "ready" })

	self.onmessage = async (event) => {
		switch (event.data.type) {
			case "addFile":
				runtime.send_program(event.data.name, event.data.content);
				break;
			case "parseWithMainAs":
				runtime.parse_with_main(event.data.name);
				break;
			case "evaluate":
				expression = event.data.expression
				const error = await runtime.evaluations(
					expression,
					(result, timeElapsed) => {
						self.postMessage({
							type: "result",
							result,
							timeElapsed,
						})
					},
					() => {
						self.postMessage({ type: "done" })
					},
					self.performance
				);

				if (error) {
					self.postMessage({
						type: "error",
						error,
					})
				}

				break;
		}
	}

}

init_wasm();
