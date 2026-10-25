import { DEFAULT_HISTORY_BOUND } from "editor";

import { HISTORY_BOUND_FIELDS } from "./history-bound-fields";
import { HISTORY_BOUND_KEY } from "./history-bound-key";
import { readBoundPart } from "./read-bound-part";
import { readBrowserStorage } from "./read-browser-storage";

import type { HistoryBound } from "editor";

/**
 * Reads the value a text kept in the browser holds; a broken text gives none.
 *
 * @kind procedure
 * @param written - the text kept in the storage
 * @returns the value it holds, or none
 */
function readWrittenValue(written: string): unknown {
  try {
    return JSON.parse(written);
  } catch {
    return undefined;
  }
}

/**
 * Reads a part of the bound kept in the browser, if its field accepts it (`readBoundPart`),
 * otherwise its default.
 *
 * @kind procedure
 * @param written - the object kept in the browser
 * @param part - `size` or `steps`
 * @returns the part, in bytes or in steps
 */
function readStoredPart(written: object, part: keyof HistoryBound): number {
  const value: unknown = Reflect.get(written, part);

  return readBoundPart(value, HISTORY_BOUND_FIELDS[part]) ?? DEFAULT_HISTORY_BOUND[part];
}

/**
 * Reads the history's bound kept in this browser (`HISTORY_BOUND_KEY`), each part its
 * field accepts kept (`readBoundPart`), any other back to its default, 100 MB or 5 steps; without
 * the browser's storage, or with a text it did not write, the default (SP-029's answer 6 (i)).
 *
 * @kind procedure
 * @param document - page document
 * @returns the memory size and the steps kept
 */
export function readStoredHistoryBound(document: Document): HistoryBound {
  const text = readBrowserStorage(document)?.getItem(HISTORY_BOUND_KEY) ?? "{}";
  const value = readWrittenValue(text);
  const written = value instanceof Object ? value : {};

  return { size: readStoredPart(written, "size"), steps: readStoredPart(written, "steps") };
}
