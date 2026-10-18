import type { FontFace } from "../font";

/**
 * The key of a face's style within its family, its weight and whether it is italic, the two
 * descriptors a family's faces differ by here (`REF-CSS-FONTS-4`, §4: `font-weight`,
 * `font-style`): two faces of one family are one when their keys are equal. 400 upright is
 * `400/normal`, 700 italic `700/italic`.
 *
 * @kind domain
 * @param face - the face's weight and style
 * @returns its key
 * @see REF-CSS-FONTS-4
 */
export function faceStyleKey(face: Pick<FontFace, "isItalic" | "weight">): string {
  return `${String(face.weight)}/${face.isItalic ? "italic" : "normal"}`;
}
