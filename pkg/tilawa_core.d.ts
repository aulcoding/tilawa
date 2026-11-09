/* tslint:disable */
/* eslint-disable */

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
    readonly __wbg_tilawa_free: (a: number, b: number) => void;
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
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
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
