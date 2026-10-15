import { selectElement } from "../page";

import type { DrawnTextRecord } from "./drawn-text-record";

/**
 * Reads the text last drawn, kept by `writeTextPosition` (US-038).
 *
 * @kind procedure
 * @param document - page document
 * @returns it, nothing before a first text is drawn
 * @see docs/backlog/stories/E15-F05-US-038-a-turned-text-grows-in-place.md
 */
export function readDrawnText(document: Document): DrawnTextRecord | undefined {
  const { drawnText } = selectElement(document, "text-options").dataset;

  // eslint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- written by writeTextPosition only, from a valid text
  return drawnText === undefined ? undefined : (JSON.parse(drawnText) as DrawnTextRecord);
}
