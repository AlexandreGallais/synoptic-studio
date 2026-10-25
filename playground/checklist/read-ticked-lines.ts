import { readBrowserStorage } from "./read-browser-storage";
import { readChecklistKey } from "./read-checklist-key";

/**
 * Reads a value written as JSON; a broken text gives none.
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
 * Reads a list of texts written as JSON; anything else, a broken text or another value, gives none.
 *
 * @kind procedure
 * @param written - the text kept in the storage
 * @returns the texts listed
 */
function readTextList(written: string): readonly string[] {
  const value = readWrittenValue(written);
  const items: readonly unknown[] = Array.isArray(value) ? value : [];

  return items.filter((item) => typeof item === "string");
}

/**
 * Reads the lines of a checklist ticked in this browser, by what each line says to try. Without
 * the browser's storage (blocked, private window) or with a value it did not write, none.
 *
 * @kind procedure
 * @param document - the playground's document
 * @param name - the checklist's name, `E15`
 * @returns what the ticked lines say to try
 */
export function readTickedLines(document: Document, name: string): ReadonlySet<string> {
  const written = readBrowserStorage(document)?.getItem(readChecklistKey(name)) ?? "[]";

  return new Set(readTextList(written));
}
