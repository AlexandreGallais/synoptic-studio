import { isSameFamilyName } from "../../src";

import { addFontFile } from "./add-font-file";
import { readFontSession } from "./read-font-session";
import { writeFontOptions } from "./write-font-options";
import { writeFontSession } from "./write-font-session";

import type { InstalledFont } from "./installed-font";
import type { NamedFontFace } from "../../src";

/** Why a file read is refused: it is not a TrueType or OpenType font. */
const NOT_A_FONT = "not a TrueType or OpenType font, a collection among them";

/**
 * Reads the file of one installed face, as the browser gives it (`blob()`,
 * `REF-WICG-LOCAL-FONT-ACCESS`), into a face added to the session's families.
 *
 * @kind procedure
 * @param document - page document
 * @param font - a face the browser lists
 * @param update - redraws the playground
 * @returns the face and its family; undefined for a file that is not a TrueType or OpenType font
 * @throws {DOMException} when the browser cannot give the file
 * @see REF-W3C-FILE-API
 */
async function readInstalledFace(
  document: Document,
  font: InstalledFont,
  update: () => void,
): Promise<NamedFontFace | undefined> {
  const blob = await font.blob();
  const bytes = await blob.arrayBuffer();

  return addFontFile(document, new Uint8Array(bytes), update);
}

/**
 * Reads why one face was refused: the browser's error, or {@link NOT_A_FONT} for a file read that
 * is not a font.
 *
 * @kind procedure
 * @param read - the read of its file, fulfilled or rejected
 * @returns the reason; empty for a face read
 * @see REF-WICG-LOCAL-FONT-ACCESS
 */
function readRefusal(read: PromiseSettledResult<NamedFontFace | undefined>): string {
  if (read.status === "rejected") {
    return String(read.reason);
  }

  return read.value === undefined ? NOT_A_FONT : "";
}

/**
 * Reads the faces whose files were read and are fonts.
 *
 * @kind procedure
 * @param settled - the reads of a family's faces, fulfilled or rejected
 * @returns the faces kept, in the browser's order
 * @see REF-WICG-LOCAL-FONT-ACCESS
 */
function readKeptFaces(
  settled: readonly PromiseSettledResult<NamedFontFace | undefined>[],
): readonly NamedFontFace[] {
  return settled.flatMap((read) =>
    read.status === "fulfilled" && read.value !== undefined ? [read.value] : [],
  );
}

/**
 * Reads why faces of a family were refused, each reason once (`readRefusal`).
 *
 * @kind procedure
 * @param settled - the reads of its faces, fulfilled or rejected
 * @returns the reasons, joined; empty when none was refused
 * @see REF-WICG-LOCAL-FONT-ACCESS
 */
function readRefusals(settled: readonly PromiseSettledResult<NamedFontFace | undefined>[]): string {
  const reasons = settled.map((read) => readRefusal(read));

  return [...new Set(reasons.filter((reason) => reason !== ""))].join("; ");
}

/**
 * Reads what the page says once a family's faces are read: how many were read and refused, and
 * why; when none was read, that the family chosen before it is kept.
 *
 * @kind procedure
 * @param family - the installed family chosen
 * @param faces - those of its faces that were read and are fonts
 * @param settled - the reads of its faces, fulfilled or rejected
 * @returns such as `Arial: 4 faces read.`
 * @see REF-WICG-LOCAL-FONT-ACCESS
 */
function readFamilyMessage(
  family: string,
  faces: readonly NamedFontFace[],
  settled: readonly PromiseSettledResult<NamedFontFace | undefined>[],
): string {
  const refused = settled.length - faces.length;
  const why = readRefusals(settled);

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
 * kept. When no face is read, the list goes back to the family the text is measured with.
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
  const settled = await Promise.allSettled(
    fonts.map(async (font) => await readInstalledFace(document, font, update)),
  );
  const faces = readKeptFaces(settled);
  const [first] = faces;
  const session = readFontSession(document);

  writeFontSession(document, {
    ...session,
    installed: installed.filter((font) => !isSameFamilyName(font.family, family)),
  });

  writeFontOptions(document, first === undefined ? session.measured : first.family);

  return readFamilyMessage(family, faces, settled);
}
