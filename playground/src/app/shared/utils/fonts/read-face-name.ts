import type { FontFace } from "editor";

/**
 * Reads the name of a face as the page says it: its family, its weight, and `italic` for an italic face.
 *
 * @kind procedure
 * @param family - its family's name
 * @param face - its weight and style
 * @returns such as `Arial 700 italic`
 */
export function readFaceName(family: string, face: Pick<FontFace, "isItalic" | "weight">): string {
  return `${family} ${String(face.weight)}${face.isItalic ? " italic" : ""}`;
}
