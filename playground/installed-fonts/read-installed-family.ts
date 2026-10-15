import { isSameFamilyName } from "../../src";
import { readFontSession, writeFontOptions, writeFontSession } from "../fonts";

import { readInstalledFace } from "./read-installed-face";

import type { InstalledFaceRead } from "./installed-face-read";
import type { NamedFontFace } from "../../src";

/**
 * Reads the faces that were read and are fonts.
 *
 * @kind procedure
 * @param reads - the reads of a family's faces
 * @returns the faces kept, in the browser's order
 * @see REF-WICG-LOCAL-FONT-ACCESS
 */
function readKeptFaces(reads: readonly InstalledFaceRead[]): readonly NamedFontFace[] {
  return reads.flatMap((read) => (read.face === undefined ? [] : [read.face]));
}

/**
 * Reads why faces of a family were refused, each reason once.
 *
 * @kind procedure
 * @param reads - the reads of its faces
 * @returns the reasons, joined; empty when none was refused
 * @see REF-WICG-LOCAL-FONT-ACCESS
 */
function readRefusals(reads: readonly InstalledFaceRead[]): string {
  const reasons = reads.map((read) => read.refusal);

  return [...new Set(reasons.filter((reason) => reason !== ""))].join("; ");
}

/**
 * Reads what the page says once a family's faces are read: how many were read and refused, and
 * why; when none was read, that the family chosen before it is kept.
 *
 * @kind procedure
 * @param family - the installed family chosen
 * @param faces - those of its faces that were read and are fonts
 * @param reads - the reads of its faces
 * @returns such as `Arial: 4 faces read.`
 * @see REF-WICG-LOCAL-FONT-ACCESS
 */
function readFamilyMessage(
  family: string,
  faces: readonly NamedFontFace[],
  reads: readonly InstalledFaceRead[],
): string {
  const refused = reads.length - faces.length;
  const why = readRefusals(reads);

  if (faces.length === 0) {
    return `${family} could not be read (${why}).`;
  }

  return refused === 0
    ? `${family}: ${String(faces.length)} faces read.`
    : `${family}: ${String(faces.length)} faces read, ${String(refused)} refused (${why}).`;
}

/**
 * Reads the files of an installed family chosen in the font list, once, each face on its own: a
 * file read as an imported one is (EN-039), the family then kept with its metrics, no longer
 * listed as installed, and chosen under the name its files give; a face whose file the browser
 * cannot give, or that is not a TrueType or OpenType font, is refused and counted, the others
 * kept. A face held in a collection is read from it (`readInstalledFace`), one face per entry of
 * the browser's list, so that a collection holding faces of several families gives the family
 * chosen only. When no face is read, the list goes back to the family the text is measured with.
 *
 * @kind procedure
 * @param document - page document
 * @param family - the installed family chosen
 * @param update - redraws the playground
 * @returns what was read and refused, such as `Arial: 4 faces read.`
 * @see REF-WICG-LOCAL-FONT-ACCESS
 */
export async function readInstalledFamily(
  document: Document,
  family: string,
  update: () => void,
): Promise<string> {
  const { installed } = readFontSession(document);
  const fonts = installed.filter((font) => isSameFamilyName(font.family, family));
  const reads = await Promise.all(
    fonts.map(async (font) => await readInstalledFace(document, font, update)),
  );
  const faces = readKeptFaces(reads);
  const [first] = faces;
  const session = readFontSession(document);

  writeFontSession(document, {
    ...session,
    installed: installed.filter((font) => !isSameFamilyName(font.family, family)),
  });

  writeFontOptions(document, first === undefined ? session.measured : first.family);

  return readFamilyMessage(family, faces, reads);
}
