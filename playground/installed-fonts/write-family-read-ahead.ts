import { readFontSession, writeFontSession } from "../fonts";
import { readFamilyLeftOut } from "../installed-fonts-count";

import { readFontsWithReadAheadFace } from "./read-fonts-with-read-ahead-face";
import { readKeptFaces } from "./read-kept-faces";

import type { InstalledFaceRead } from "./installed-face-read";
import type { ReadAheadFont } from "./read-ahead-font";
import type { InstalledFont } from "../fonts";
import type { FamilyOutcome } from "../installed-fonts-count";

/**
 * Reads the faces read ahead with a family's faces more, remembered under the name their files
 * give, so that they are read again when the family is chosen.
 *
 * @kind procedure
 * @param readAhead - the faces read ahead before
 * @param fonts - the family's faces, as the browser lists them
 * @param family - the name the family's files give; undefined when none of its faces was read
 * @returns the faces read ahead; those before alone for a family none of whose faces was read
 * @see docs/domain/shapes.md#text
 */
function readReadAheadFonts(
  readAhead: readonly ReadAheadFont[],
  fonts: readonly InstalledFont[],
  family: string | undefined,
): readonly ReadAheadFont[] {
  return family === undefined
    ? readAhead
    : [...readAhead, ...fonts.map((font) => ({ family, font }))];
}

/**
 * Keeps what a family's reads gave (US-053): its faces' metrics without their files
 * (`readFontsWithReadAheadFace`), its faces remembered under the name their files give so that they
 * are read again when the family is chosen, and the family no longer listed as not read; a family
 * none of whose faces is read and listed never enters the font list, its reason kept (US-056).
 *
 * @kind procedure
 * @param document - page document
 * @param fonts - the family's faces, as the browser lists them
 * @param reads - their reads
 * @returns whether the family was read, or left out and why
 * @see docs/domain/shapes.md#text
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
    readAhead: readReadAheadFonts(session.readAhead, fonts, family),
  });

  return family === undefined ? readFamilyLeftOut(reads) : "read";
}
