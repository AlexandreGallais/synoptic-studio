import type { InstalledFaceRead } from "./installed-face-read";
import type { NamedFontFace } from "editor";

/**
 * Reads the faces that were read and are fonts.
 *
 * @kind procedure
 * @param reads - the reads of a family's faces
 * @returns the faces kept, in the browser's order
 */
export function readKeptFaces(reads: readonly InstalledFaceRead[]): readonly NamedFontFace[] {
  return reads.flatMap((read) => (read.face === undefined ? [] : [read.face]));
}
