import * as wasm from "./pink_runtime_bg.wasm";
import { __wbg_set_wasm } from "./pink_runtime_bg.js";
__wbg_set_wasm(wasm);
export * from "./pink_runtime_bg.js";
