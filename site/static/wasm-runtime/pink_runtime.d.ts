declare namespace wasm_bindgen {
	/* tslint:disable */
	/* eslint-disable */
	/**
	*/
	export class AsyncRuntime {
	  free(): void;
	/**
	* @returns {AsyncRuntime}
	*/
	  static new(): AsyncRuntime;
	/**
	* @param {string} name
	* @param {string} program
	*/
	  send_program(name: string, program: string): void;
	/**
	* @param {string} name
	*/
	  parse_with_main(name: string): void;
	/**
	* @param {string} expression
	* @param {Function} callback
	* @param {Function} on_finish
	* @param {Performance} performance
	* @returns {Promise<any>}
	*/
	  evaluations(expression: string, callback: Function, on_finish: Function, performance: Performance): Promise<any>;
	}
	
}

declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_asyncruntime_free: (a: number) => void;
  readonly asyncruntime_new: () => number;
  readonly asyncruntime_send_program: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly asyncruntime_parse_with_main: (a: number, b: number, c: number) => void;
  readonly asyncruntime_evaluations: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h1ef8d92bd332269a: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__h0ca52d7172d7c26f: (a: number, b: number, c: number, d: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
declare function wasm_bindgen (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
