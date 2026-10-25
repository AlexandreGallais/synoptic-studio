import { fontsWithFace, isSameFamilyName } from "../../src";

import type { FontFace, FontFamily, NamedFontFace } from "../../src";

/**
 * Reads the kept faces that hold their file and have the weight and style of a face read.
 *
 * @kind procedure
 * @param faces - the faces of the family kept
 * @param face - the face read ahead
 * @returns those faces; none when no face of that weight and style holds its file
 * @see RULE-shapes-text
 */
function readHeldFaces(faces: readonly FontFace[], face: FontFace): readonly FontFace[] {
  return faces
    .filter((kept) => kept.file !== undefined)
    .filter((kept) => kept.weight === face.weight)
    .filter((kept) => kept.isItalic === face.isItalic);
}

/**
 * Reads the font families kept with a face read ahead (US-053): its metrics added without its file
 * (`fontsWithFace`), replacing a face of the same weight and style whose file is not held — one
 * of Inter's shipped faces —, never one whose file is held, imported or read when its family was
 * chosen, so that no file the page draws and embeds is dropped.
 *
 * @kind procedure
 * @param fonts - the font families kept
 * @param named - the face read ahead, its file held, and its family
 * @returns the families, that of the face holding its metrics; the families given when a face of that weight and style holds its file
 * @see RULE-shapes-text
 */
export function readFontsWithReadAheadFace(
  fonts: readonly FontFamily[],
  named: NamedFontFace,
): readonly FontFamily[] {
  const faces = fonts.find((kept) => isSameFamilyName(kept.name, named.family))?.faces ?? [];

  return readHeldFaces(faces, named.face).length > 0
    ? fonts
    : fontsWithFace(fonts, { face: { ...named.face, file: undefined }, family: named.family });
}
