import { selectElement } from "../page";

import type { DrawnTextRecord } from "./drawn-text-record";

/**
 * Reads the text last drawn, kept by `writeTextPosition`.
 *
 * @kind procedure
 * @param document - page document
 * @returns it, nothing before a first text is drawn
 */
export function readDrawnText(document: Document): DrawnTextRecord | undefined {
  const { drawnText } = selectElement(document, "text-options").dataset;

  // oxlint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- written by writeTextPosition only, from a valid text
  return drawnText === undefined ? undefined : (JSON.parse(drawnText) as DrawnTextRecord);
}
