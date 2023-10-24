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
