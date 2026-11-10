/* tslint:disable */
/* eslint-disable */
/**
 * The `ReadableStreamType` enum.
 *
 * *This API requires the following crate features to be activated: `ReadableStreamType`*
 */

export type ReadableStreamType = "bytes";

export class IntoUnderlyingByteSource {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    cancel(): void;
    pull(controller: ReadableByteStreamController): Promise<any>;
    start(controller: ReadableByteStreamController): void;
    readonly autoAllocateChunkSize: number;
    readonly type: ReadableStreamType;
}

export class IntoUnderlyingSink {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    abort(reason: any): Promise<any>;
    close(): Promise<any>;
    write(chunk: any): Promise<any>;
}

export class IntoUnderlyingSource {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    cancel(): void;
    pull(controller: ReadableStreamDefaultController): Promise<any>;
}

/**
 * The handle the page's JavaScript holds.
 */
export class ScheduleApp {
    free(): void;
    [Symbol.dispose](): void;
    finished_count(): number;
    /**
     * Renders the 29 cards into `grid` and the progress header into `head`,
     * restoring saved progress from localStorage.
     */
    constructor(grid: HTMLElement, head: HTMLElement, texts_json: string, on_event: Function);
    /**
     * New language: every label updates in place (no re-render, so moons keep their state).
     * The header gets a fresh line in the new language and the reset button disarms.
     */
    set_texts(texts_json: string): void;
}

export class Tilawa {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Returns an outcome as JSON, e.g. {"kind":"finished"} or {"kind":"locked","next":1}.
     */
    add_page(day: number): string;
    can_add(day: number): boolean;
    can_remove(day: number): boolean;
    /**
     * The 29 days as JSON: [{ s, r, tag?, pg }, ...]
     */
    days_json(): string;
    finished_count(): number;
    /**
     * For the older localStorage "tilawa-progress-v1".
     */
    finished_days_json(): string;
    /**
     * Pages read per day (index 0 = Day 1). Arrives in JS as a Uint16Array.
     */
    pages(): Uint16Array;
    /**
     * Same shape as add_page, e.g. {"kind":"undofromend","last":2}.
     */
    remove_page(day: number): string;
    reset(): void;
    /**
     * Starts from saved data (either argument may be null).
     */
    static restore(saved_pages?: string | null, old_done?: string | null): Tilawa;
    /**
     * For localStorage "tilawa-pages-v1".
     */
    to_json(): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_intounderlyingbytesource_free: (a: number, b: number) => void;
    readonly __wbg_intounderlyingsink_free: (a: number, b: number) => void;
    readonly __wbg_intounderlyingsource_free: (a: number, b: number) => void;
    readonly __wbg_scheduleapp_free: (a: number, b: number) => void;
    readonly __wbg_tilawa_free: (a: number, b: number) => void;
    readonly intounderlyingbytesource_autoAllocateChunkSize: (a: number) => number;
    readonly intounderlyingbytesource_cancel: (a: number) => void;
    readonly intounderlyingbytesource_pull: (a: number, b: any) => any;
    readonly intounderlyingbytesource_start: (a: number, b: any) => void;
    readonly intounderlyingbytesource_type: (a: number) => number;
    readonly intounderlyingsink_abort: (a: number, b: any) => any;
    readonly intounderlyingsink_close: (a: number) => any;
    readonly intounderlyingsink_write: (a: number, b: any) => any;
    readonly intounderlyingsource_cancel: (a: number) => void;
    readonly intounderlyingsource_pull: (a: number, b: any) => any;
    readonly scheduleapp_finished_count: (a: number) => number;
    readonly scheduleapp_new: (a: any, b: any, c: number, d: number, e: any) => number;
    readonly scheduleapp_set_texts: (a: number, b: number, c: number) => void;
    readonly tilawa_add_page: (a: number, b: number) => [number, number];
    readonly tilawa_can_add: (a: number, b: number) => number;
    readonly tilawa_can_remove: (a: number, b: number) => number;
    readonly tilawa_days_json: (a: number) => [number, number];
    readonly tilawa_finished_count: (a: number) => number;
    readonly tilawa_finished_days_json: (a: number) => [number, number];
    readonly tilawa_pages: (a: number) => [number, number];
    readonly tilawa_remove_page: (a: number, b: number) => [number, number];
    readonly tilawa_reset: (a: number) => void;
    readonly tilawa_restore: (a: number, b: number, c: number, d: number) => number;
    readonly tilawa_to_json: (a: number) => [number, number];
    readonly wasm_bindgen_57db2698c03e39ef___convert__closures_____invoke___js_sys_af93aaf2a88d89c7___Function_fn_wasm_bindgen_57db2698c03e39ef___JsValue_____wasm_bindgen_57db2698c03e39ef___sys__Undefined___js_sys_af93aaf2a88d89c7___Function_fn_wasm_bindgen_57db2698c03e39ef___JsValue_____wasm_bindgen_57db2698c03e39ef___sys__Undefined_______true_: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen_57db2698c03e39ef___convert__closures_____invoke___wasm_bindgen_57db2698c03e39ef___JsValue__core_23c8f662a870ef55___result__Result_____wasm_bindgen_57db2698c03e39ef___JsError___true_: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_57db2698c03e39ef___convert__closures_____invoke___wasm_bindgen_57db2698c03e39ef___JsValue______true_: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_57db2698c03e39ef___convert__closures_____invoke___wasm_bindgen_57db2698c03e39ef___JsValue______true__27: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_57db2698c03e39ef___convert__closures_____invoke_______true_: (a: number, b: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_destroy_closure: (a: number, b: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
