import { distinctFamilyNames, isSameFamilyName } from "../../src";
import { NO_FONT_APPROVED } from "../default-font";
import { readFontSession } from "../fonts";
import { readCountWith, readProgressLine } from "../installed-fonts-count";
import { writeText } from "../page";

import { readFacesInTurn } from "./read-faces-in-turn";
import { writeDefaultChoice } from "./write-default-choice";
import { writeFamilyReadAhead } from "./write-family-read-ahead";

import type { ReadAheadCount } from "../installed-fonts-count";

/** The page the installed fonts are read ahead for. */
type ReadAheadPage = {
  /** Page document. */
  readonly document: Document;
  /** Its window, which runs the tasks waiting between two files. */
  readonly view: Window;
};

/**
 * Reads the installed families not read yet, one after the other, each said and redrawn as it is
 * read, until none is left.
 *
 * @kind procedure
 * @param page - the page and its window
 * @param update - redraws the playground
 * @param count - what was counted before
 * @returns what was counted once every family is done
 * @see docs/domain/shapes.md#text
 */
async function readNextFamilies(
  page: ReadAheadPage,
  update: () => void,
  count: ReadAheadCount,
): Promise<ReadAheadCount> {
  const { installed } = readFontSession(page.document);
  const [next] = installed;

  if (next === undefined) {
    return count;
  }

  const fonts = installed.filter((font) => isSameFamilyName(font.family, next.family));
  const reads = await readFacesInTurn(page.view, fonts);
  const counted = readCountWith(
    count,
    next.family,
    writeFamilyReadAhead(page.document, fonts, reads),
  );

  writeText(page.document, "installed-fonts-progress", readProgressLine(counted, false));
  update();

  return await readNextFamilies(page, update, counted);
}

/**
 * Reads the installed fonts the browser listed in the background, family after family, each face
 * on its own, the page answering between two files (US-053): a family keeps its faces' metrics,
 * not their files, which are read again when it is chosen; a family none of whose faces can be
 * read and approved never enters the font list; a line counts the families read, then says those
 * left out. Once all are read, the first family read and approved is chosen when no font is
 * available, its files read again (`writeDefaultChoice`, US-056, US-060), else the page says that
 * none can be used (US-059).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @returns once every installed family is read
 * @see docs/domain/shapes.md#text
 */
export async function readInstalledFontsAhead(
  document: Document,
  update: () => void,
): Promise<void> {
  const view = document.defaultView;

  if (view === null) {
    return;
  }

  const { installed } = readFontSession(document);
  const total = distinctFamilyNames(installed.map((font) => font.family)).length;
  const start: ReadAheadCount = { done: 0, leftOut: [], read: 0, total };

  writeText(document, "installed-fonts-progress", readProgressLine(start, false));

  const count = await readNextFamilies({ document, view }, update, start);

  writeText(document, "installed-fonts-progress", readProgressLine(count, true));
  await writeDefaultChoice(document, update);

  if (readFontSession(document).kept.length === 0) {
    writeText(document, "font-message", NO_FONT_APPROVED);
  }

  update();
}
