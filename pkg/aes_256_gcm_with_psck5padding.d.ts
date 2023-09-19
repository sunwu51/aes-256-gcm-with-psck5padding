declare namespace wasm_bindgen {
	/* tslint:disable */
	/* eslint-disable */
	/**
	* @param {string} key
	* @param {string} plaintext
	* @returns {string}
	*/
	export function aes_256_gcm_with_psck5padding_encrypt(key: string, plaintext: string): string;
	/**
	* @param {string} key
	* @param {string} base64_str
	* @returns {string}
	*/
	export function aes_256_gcm_with_psck5padding_decrypt(key: string, base64_str: string): string;
	
}

declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly aes_256_gcm_with_psck5padding_encrypt: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly aes_256_gcm_with_psck5padding_decrypt: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
