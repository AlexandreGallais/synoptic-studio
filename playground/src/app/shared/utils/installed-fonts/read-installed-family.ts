import { addFontFace, readFontSession, writeFontOptions, writeFontSession } from "../fonts";

import { readFamilyFonts } from "./read-family-fonts";
import { readFontsWithoutFacesReadAhead } from "./read-fonts-without-faces-read-ahead";
import { readInstalledFace } from "./read-installed-face";
import { readKeptFaces } from "./read-kept-faces";

import type { InstalledFaceRead } from "./installed-face-read";
import type { NamedFontFace } from "editor";

/**
 * Reads why faces of a family were refused, each reason once.
 *
 * @kind procedure
 * @param reads - the reads of its faces
 * @returns the reasons, joined; empty when none was refused
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
 * Reads the files of an installed family chosen in the font list, each face on its own: a file
 * read as an imported one is, the family then kept with its metrics and files, so that
 * the page draws it with its own outlines and can embed it, no longer listed as installed, and
 * chosen under the name its files give; a face whose file the browser cannot give, or that is not
 * a TrueType or OpenType font, is refused and counted, the others kept. A face held in a
 * collection is read from it (`readInstalledFace`), one face per entry of the browser's list, so
 * that a collection holding faces of several families gives the family chosen only. A family read
 * ahead, its metrics kept without its files, has its files read again; a face refused at
 * that second reading has its metrics read ahead dropped (`readFontsWithoutFacesReadAhead`), so
 * that it is no longer measured, the refusal said. When no face is
 * read again, the list stays on the family, whose metrics read ahead are kept: a family is offered
 * only once read ahead.
 *
 * @kind procedure
 * @param document - page document
 * @param family - the installed family chosen
 * @param update - redraws the playground
 * @returns what was read and refused, such as `Arial: 4 faces read.`
 */
export async function readInstalledFamily(
  document: Document,
  family: string,
  update: () => void,
): Promise<string> {
  const fonts = readFamilyFonts(readFontSession(document), family);
  const reads = await Promise.all(fonts.map(async (font) => await readInstalledFace(font)));
  const faces = readKeptFaces(reads);

  for (const face of faces) {
    addFontFace(document, face, update);
  }

  const session = readFontSession(document);

  writeFontSession(document, {
    ...session,
    kept: readFontsWithoutFacesReadAhead(session.kept, faces),
    readAhead: session.readAhead.filter((read) => !fonts.includes(read.font)),
  });

  writeFontOptions(document, faces[0]?.family ?? family);

  return readFamilyMessage(family, faces, reads);
}
