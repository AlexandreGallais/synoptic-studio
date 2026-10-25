import { readFontSession, writeFontSession } from "../fonts";
import { readFamilyLeftOut } from "../installed-fonts-count";

import { readFontsWithReadAheadFace } from "./read-fonts-with-read-ahead-face";
import { readKeptFaces } from "./read-kept-faces";

import type { InstalledFaceRead } from "./installed-face-read";
import type { ReadAheadFont } from "./read-ahead-font";
import type { InstalledFont } from "../fonts";
import type { FamilyOutcome } from "../installed-fonts-count";

/**
 * Reads the faces read ahead with a family's faces more, those read and checked alone, remembered
 * under the name their files give, so that they are read again when the family is chosen; a face
 * refused at this first reading is never read again nor listed.
 *
 * @kind procedure
 * @param readAhead - the faces read ahead before
 * @param fonts - the family's faces, as the browser lists them
 * @param reads - their reads, in the same order
 * @returns the faces read ahead; those before alone for a family none of whose faces was read
 * @see RULE-shapes-text
 */
function readReadAheadFonts(
  readAhead: readonly ReadAheadFont[],
  fonts: readonly InstalledFont[],
  reads: readonly InstalledFaceRead[],
): readonly ReadAheadFont[] {
  const family = readKeptFaces(reads)[0]?.family;

  return family === undefined
    ? readAhead
    : [
        ...readAhead,
        ...fonts
          .filter((font, index) => reads[index]?.face !== undefined)
          .map((font) => ({ family, font })),
      ];
}

/**
 * Keeps what a family's reads gave: its faces' metrics without their files
 * (`readFontsWithReadAheadFace`), its faces read and checked remembered under the name their files
 * give so that they are read again when the family is chosen, a face refused left out,
 * and the family no longer listed as not read; a family none of whose faces is read and listed
 * never enters the font list, its reason kept.
 *
 * @kind procedure
 * @param document - page document
 * @param fonts - the family's faces, as the browser lists them
 * @param reads - their reads
 * @returns whether the family was read, or left out and why
 * @see RULE-shapes-text
 */
export function writeFamilyReadAhead(
  document: Document,
  fonts: readonly InstalledFont[],
  reads: readonly InstalledFaceRead[],
): FamilyOutcome {
  const session = readFontSession(document);
  const faces = readKeptFaces(reads);
  const family = faces[0]?.family;

  writeFontSession(document, {
    ...session,
    installed: session.installed.filter((font) => !fonts.includes(font)),
    kept: faces.reduce((kept, face) => readFontsWithReadAheadFace(kept, face), session.kept),
    readAhead: readReadAheadFonts(session.readAhead, fonts, reads),
  });

  return family === undefined ? readFamilyLeftOut(reads) : "read";
}
