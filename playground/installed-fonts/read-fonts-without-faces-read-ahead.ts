import { DEFAULT_FONT, isSameFamilyName } from "../../src";

import type { FontFace, FontFamily, NamedFontFace } from "../../src";

/**
 * Reads the faces of a family read again that are held: their file held, or Inter's shipped
 * faces, which the browser draws as their metrics measure them.
 *
 * @kind procedure
 * @param faces - the faces kept of the family read again
 * @returns those faces, in order; none read ahead whose file was refused at the second reading
 */
function readHeldFaces(faces: readonly FontFace[]): readonly FontFace[] {
  return faces.filter((face) => face.file !== undefined || DEFAULT_FONT.faces.includes(face));
}

/**
 * Reads the font families kept without the faces of a family read again whose files were refused
 * at that second reading: their metrics read ahead dropped (`readHeldFaces`), so that a
 * text asking such a face is measured with the nearest face held, as for a weight the family
 * lacks, and the face is never measured while the browser no longer gives it, nor left out of an
 * export, its file not held. When none of its faces is read again, the family keeps its metrics
 * read ahead, the list staying on it.
 *
 * @kind procedure
 * @param fonts - the font families kept
 * @param faces - the faces read again, their files held, and their family
 * @returns the families, that of the faces holding its faces held alone; the families given when no face is read again
 */
export function readFontsWithoutFacesReadAhead(
  fonts: readonly FontFamily[],
  faces: readonly NamedFontFace[],
): readonly FontFamily[] {
  const [first] = faces;

  return first === undefined
    ? fonts
    : fonts.map((kept) =>
        isSameFamilyName(kept.name, first.family)
          ? { ...kept, faces: readHeldFaces(kept.faces) }
          : kept,
      );
}
