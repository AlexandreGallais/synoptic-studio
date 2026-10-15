import { isSameFamilyName } from "../../src";

import { addFontFile } from "./add-font-file";
import { readFontSession } from "./read-font-session";
import { writeFontOptions } from "./write-font-options";
import { writeFontSession } from "./write-font-session";

import type { InstalledFont } from "./installed-font";
import type { NamedFontFace } from "../../src";

/**
 * Reads the file of one installed face, as the browser gives it (`blob()`,
 * `REF-WICG-LOCAL-FONT-ACCESS`), into a face added to the session's families.
 *
 * @kind procedure
 * @param document - page document
 * @param font - a face the browser lists
 * @param update - redraws the playground
 * @returns the face and its family; undefined for a file that is not a TrueType or OpenType font
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
 * Reads the files of an installed family chosen in the font list, once: each face's file read as
 * an imported one is (EN-039), the family then kept with its metrics and no longer listed as
 * installed, and chosen under the name its files give; a face whose file is not a TrueType or
 * OpenType font, a collection among them, is left out and counted.
 *
 * @kind procedure
 * @param document - page document
 * @param family - the installed family chosen
 * @param update - redraws the playground
 * @returns what was read, such as `Arial: 4 faces read.`
 * @see REF-WICG-LOCAL-FONT-ACCESS
 */
export async function readInstalledFamily(
  document: Document,
  family: string,
  update: () => void,
): Promise<string> {
  const { installed } = readFontSession(document);
  const fonts = installed.filter((font) => isSameFamilyName(font.family, family));
  const read = await Promise.all(
    fonts.map(async (font) => await readInstalledFace(document, font, update)),
  );
  const faces = read.filter((face) => face !== undefined);
  const refused = read.length - faces.length;

  writeFontSession(document, {
    ...readFontSession(document),
    installed: installed.filter((font) => !isSameFamilyName(font.family, family)),
  });

  writeFontOptions(document, faces[0]?.family ?? "");

  return `${family}: ${String(faces.length)} faces read${refused > 0 ? `, ${String(refused)} refused (not a TrueType or OpenType font, a collection among them)` : ""}.`;
}
