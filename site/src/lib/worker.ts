import { AsyncRuntime } from "$runtime";

const runtime = AsyncRuntime.new();

export let expression = "";

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
				(result: string, timeElapsed: number) => {
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
