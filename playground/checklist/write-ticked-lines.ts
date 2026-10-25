import { readBrowserStorage } from "./read-browser-storage";
import { readChecklistKey } from "./read-checklist-key";

/**
 * Keeps the lines of a checklist ticked in this browser, by what each line says to try. Without
 * the browser's storage, or with a storage full, nothing is kept and the page goes on: the ticks
 * are lost at the reload.
 *
 * @kind procedure
 * @param document - the playground's document
 * @param name - the checklist's name, `E15`
 * @param todos - what the ticked lines say to try
 * @see US-051
 */
export function writeTickedLines(document: Document, name: string, todos: readonly string[]): void {
  const storage = readBrowserStorage(document);

  if (storage === undefined) {
    return;
  }

  const written = JSON.stringify(todos);

  try {
    storage.setItem(readChecklistKey(name), written);
  } catch {
    // The storage is a convenience: the page works without it.
  }
}
