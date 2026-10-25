import { DEFAULT_FONT, isSameFamilyName } from "../../src";
import { readFontSession, writeFontOptions, writeFontSession } from "../fonts";
import { writeText } from "../page";

import { readInstalledFamily } from "./read-installed-family";

import type { FontFace, FontFamily } from "../../src";

/**
 * Reads the faces of a kept family that hold their file, imported or read again when the family
 * was chosen, so that the page can embed them (US-060).
 *
 * @kind procedure
 * @param family - the family; undefined when none is kept under its name
 * @returns those faces, in the family's order; none for a family read ahead or not kept
 * @see RULE-shapes-text
 */
function readFacesWithFile(family: FontFamily | undefined): readonly FontFace[] {
  return family?.faces.filter((face) => face.file !== undefined) ?? [];
}

/**
 * Reads the faces of the family the text is measured with that make a font available once the
 * installed fonts are read: Inter's, drawn by the browser; any other family's that hold their
 * file, imported or read again when chosen (`readFacesWithFile`). A family read ahead the text was
 * measured with only because the font list showed it first has none, its files not held (US-060).
 *
 * @kind procedure
 * @param family - the kept family the text is measured with; undefined when it is not kept
 * @returns those faces; none when no font is available
 * @see RULE-shapes-text
 */
function readAvailableFaces(family: FontFamily | undefined): readonly FontFace[] {
  return family !== undefined && isSameFamilyName(family.name, DEFAULT_FONT.name)
    ? family.faces
    : readFacesWithFile(family);
}

/**
 * Reads a kept family by its name.
 *
 * @kind procedure
 * @param document - page document
 * @param name - the family's name, matched without regard to case
 * @returns the family; undefined when none is kept under that name
 * @see RULE-shapes-text
 */
function readKeptFamily(document: Document, name: string): FontFamily | undefined {
  return readFontSession(document).kept.find((font) => isSameFamilyName(font.name, name));
}

/**
 * Reads the family to choose as the default font once the installed fonts are read: the first of
 * the list, unless the text has a font available (`readAvailableFaces`).
 *
 * @kind procedure
 * @param document - page document
 * @returns the family to choose; undefined when a font is available or none is kept
 * @see RULE-shapes-text
 */
function readFamilyToChoose(document: Document): FontFamily | undefined {
  const { kept, measured } = readFontSession(document);

  return readAvailableFaces(readKeptFamily(document, measured)).length > 0 ? undefined : kept[0];
}

/**
 * Removes a family from the families kept and the font list, none of its files read again.
 *
 * @kind procedure
 * @param document - page document
 * @param family - the family's name
 * @see RULE-shapes-text
 */
function removeKeptFamily(document: Document, family: string): void {
  const session = readFontSession(document);

  writeFontSession(document, {
    ...session,
    kept: session.kept.filter((kept) => !isSameFamilyName(kept.name, family)),
  });

  writeFontOptions(document, "");
}

/**
 * Chooses a family as the default font: the text measured with it, the font list showing it, the
 * page saying why.
 *
 * @kind procedure
 * @param document - page document
 * @param family - the family's name
 * @see RULE-shapes-text
 */
function writeChosenFamily(document: Document, family: string): void {
  writeFontSession(document, { ...readFontSession(document), measured: family });
  writeFontOptions(document, family);

  writeText(
    document,
    "font-message",
    `${family} is chosen, the first font read: Inter is not installed on this computer.`,
  );
}

/**
 * Chooses a default font once the installed fonts are read while none is available (US-056,
 * US-060): the first family of the list (`readFamilyToChoose`) is chosen as a family chosen by
 * hand is, its faces' files read again (`readInstalledFamily`) so that it can be embedded, the
 * page saying it reads them, the font list held meanwhile (US-061), then that it is chosen; a family none of whose files is read again leaves the list and the next is
 * tried. A font available, Inter drawn or a font imported, is left as it is.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @returns once a family is chosen, or none is left
 * @see RULE-shapes-text
 */
export async function writeDefaultChoice(document: Document, update: () => void): Promise<void> {
  const first = readFamilyToChoose(document);

  if (first === undefined) {
    return;
  }

  writeText(
    document,
    "font-message",
    `Reading ${first.name}, the default font: the font list can be used once it is read.`,
  );

  await readInstalledFamily(document, first.name, update);

  if (readFacesWithFile(readKeptFamily(document, first.name)).length > 0) {
    writeChosenFamily(document, first.name);

    return;
  }

  removeKeptFamily(document, first.name);
  await writeDefaultChoice(document, update);
}
