// import workerURL from './worker?url';

import { browser } from "$app/environment"
import { writable, type Writable } from "svelte/store"

export type Status = "idle" | "processing" | "invalid";

export function getWorker(): Promise<{
	addFile: (name: string, content: string) => void,
	parseWithMainAs: (name: string) => void,
	evaluate: (expression: string) => void,
	subscribe: (onResult: (result: string, timeElapsed: number) => void) => {
		unsubscribe: () => void
	},
	close: () => void,
	status: Writable<Status>,
}> {
	return new Promise((resolve, reject) => {
		if (!browser) {
			throw new Error("getWorker() is only available in the browser")
		}

		let status = writable<Status>("idle")

		const url = new URL("worker", import.meta.url)
		const worker = new Worker(url, { type: "module" })
		worker.onmessage = (event) => {
			if (event.data.type === "ready") {
				resolve(methods)
			}

			if (event.data.type === "result") {
				onResult && onResult(event.data.result, event.data.timeElapsed)
			}

			if (event.data.type === "error") {
				status.set("invalid")
			}

			if (event.data.type === "done") {
				status.set("idle")
			}
		}

		let onResult: ((result: string, timeElapsed: number) => void) | null = null;

		const methods = {
			addFile: (name: string, content: string) => {
				worker.postMessage({
					type: "addFile",
					name,
					content,
				})
			},

			parseWithMainAs: (name: string) => {
				worker.postMessage({
					type: "parseWithMainAs",
					name
				})
			},

			evaluate: (expression: string) => {
				status.set("processing")
				worker.postMessage({
					type: "evaluate",
					expression
				})
			},

			status,

			subscribe: (onResultNew: (result: string, timeElapsed: number) => void) => {
				onResult = onResultNew



				return {
					unsubscribe: () => {
						onResult = null;
					}
				}
			},

			close: () => {
				worker.terminate()
			},
		}
	})
}
