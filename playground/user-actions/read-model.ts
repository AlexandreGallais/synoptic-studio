import { readText } from "./read-text";

import type { DrawingShape } from "../../src";

/**
 * Model of the shape or the text the panel edits, as the pipeline column shows it.
 *
 * @kind procedure
 * @returns the shape or the text, parsed from `#model`
 * @see EN-021
 */
export function readModel(): DrawingShape {
  // oxlint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- the page writes the model shown by `JSON.stringify` of a shape or a text
  return JSON.parse(readText("#model")) as DrawingShape;
}
