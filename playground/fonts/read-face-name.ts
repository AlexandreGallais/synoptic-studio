import type { FontFace } from "../../src";

/**
 * Reads the name of a face as the page says it: its family, its weight, and `italic` for an italic face.
 *
 * @kind procedure
 * @param family - its family's name
 * @param face - its weight and style
 * @returns such as `Arial 700 italic`
 * @see US-036
 */
export function readFaceName(family: string, face: Pick<FontFace, "isItalic" | "weight">): string {
  return `${family} ${String(face.weight)}${face.isItalic ? " italic" : ""}`;
}
