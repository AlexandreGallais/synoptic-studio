import { isSameFamilyName } from "../font";

import { faceStyleKey } from "./face-style-key";

import type { FontFace, FontFamily } from "../font";
import type { NamedFontFace } from "./named-font-face";

/**
 * A family's faces with one face more: a face of the same weight and style (`faceStyleKey`) is
 * replaced, the last file read winning (`docs/domain/shapes.md`, Fonts from files).
 *
 * @kind domain
 * @param faces - the family's faces
 * @param face - the face added
 * @returns the faces without the one it replaces, then it
 * @see docs/domain/shapes.md#text
 */
function facesWithFace(faces: readonly FontFace[], face: FontFace): readonly FontFace[] {
  return [...faces.filter((kept) => faceStyleKey(kept) !== faceStyleKey(face)), face];
}

/**
 * The font families kept with one face more, read from a file: added to the family of its name,
 * matched without regard to case (`isSameFamilyName`), replacing a face of the same weight and
 * style, as CSS matches names (`REF-CSS-FONTS-4`, §5.1); a family not kept yet is listed last
 * with that face, and an empty version, each face's file giving its own. The families given are never changed: a new list is returned.
 *
 * @kind domain
 * @param fonts - the font families kept
 * @param named - the face read and its family's name
 * @returns the families, that of the face holding it
 * @see docs/domain/shapes.md#text
 */
export function fontsWithFace(
  fonts: readonly FontFamily[],
  named: NamedFontFace,
): readonly FontFamily[] {
  const family = fonts.find((kept) => isSameFamilyName(named.family, kept.name));

  return family === undefined
    ? [...fonts, { faces: [named.face], name: named.family, version: "" }]
    : fonts.map((kept) =>
        kept === family ? { ...kept, faces: facesWithFace(kept.faces, named.face) } : kept,
      );
}
