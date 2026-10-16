import { distinctFamilyNames, isSameFamilyName } from "../../src";
import { readFontSession, writeFontSession } from "../fonts";
import { waitForNextTask, writeText } from "../page";

import { readFontsWithReadAheadFace } from "./read-fonts-with-read-ahead-face";
import { readInstalledFace } from "./read-installed-face";
import { readKeptFaces } from "./read-kept-faces";

import type { InstalledFaceRead } from "./installed-face-read";
import type { ReadAheadFont } from "./read-ahead-font";
import type { InstalledFont } from "../fonts";

/** The page the installed fonts are read ahead for. */
type ReadAheadPage = {
  /** Page document. */
  readonly document: Document;
  /** Its window, which runs the tasks waiting between two files. */
  readonly view: Window;
};

/** What the reading ahead has counted so far. */
type ReadAheadCount = {
  /** The installed families listed when the reading started. */
  readonly total: number;
  /** The families done: read, left out, or read meanwhile because chosen. */
  readonly done: number;
  /** The families read ahead, at least one of their faces read. */
  readonly read: number;
  /** The families left out, none of their faces read, in the browser's order. */
  readonly leftOut: readonly string[];
};

/** What reading a family ahead gave: read, left out, or already read because it was chosen meanwhile. */
type FamilyOutcome = "chosen" | "left out" | "read";

/**
 * Reads the faces of a family one after the other (`readInstalledFace`), the page answering before
 * each file is read (`waitForNextTask`), so that a family of large files never freezes it.
 *
 * @kind procedure
 * @param view - the page's window
 * @param fonts - the faces, as the browser lists them
 * @returns their reads, in the same order
 * @see REF-WEBDEV-LONG-TASKS
 */
async function readFacesInTurn(
  view: Window,
  fonts: readonly InstalledFont[],
): Promise<readonly InstalledFaceRead[]> {
  const [font, ...rest] = fonts;

  if (font === undefined) {
    return [];
  }

  await waitForNextTask(view);

  const read = await readInstalledFace(font);

  return [read, ...(await readFacesInTurn(view, rest))];
}

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
 * none of whose faces is read leaves the font list. A family chosen meanwhile, read when chosen, is
 * left as it is.
 *
 * @kind procedure
 * @param document - page document
 * @param fonts - the family's faces, as the browser lists them
 * @param reads - their reads
 * @returns whether the family was read, left out, or chosen meanwhile
 * @see docs/domain/shapes.md#text
 */
function writeFamilyReadAhead(
  document: Document,
  fonts: readonly InstalledFont[],
  reads: readonly InstalledFaceRead[],
): FamilyOutcome {
  const session = readFontSession(document);

  if (fonts.every((font) => !session.installed.includes(font))) {
    return "chosen";
  }

  const faces = readKeptFaces(reads);
  const family = faces[0]?.family;

  writeFontSession(document, {
    ...session,
    installed: session.installed.filter((font) => !fonts.includes(font)),
    kept: faces.reduce((kept, face) => readFontsWithReadAheadFace(kept, face), session.kept),
    readAhead: readReadAheadFonts(session.readAhead, fonts, family),
  });

  return family === undefined ? "left out" : "read";
}

/**
 * Counts one family more.
 *
 * @kind procedure
 * @param count - what was counted before it
 * @param family - its name, as the browser lists it
 * @param outcome - what reading it gave
 * @returns the count with it
 * @see docs/domain/shapes.md#text
 */
function readCountWith(
  count: ReadAheadCount,
  family: string,
  outcome: FamilyOutcome,
): ReadAheadCount {
  return {
    ...count,
    done: count.done + 1,
    leftOut: outcome === "left out" ? [...count.leftOut, family] : count.leftOut,
    read: outcome === "read" ? count.read + 1 : count.read,
  };
}

/**
 * Reads what the page says while the installed fonts are read, then once they are.
 *
 * @kind procedure
 * @param count - what was counted
 * @param isDone - whether every family is done
 * @returns such as `Reading the installed fonts: 12 of 312 families…`
 * @see docs/domain/shapes.md#text
 */
function readProgressLine(count: ReadAheadCount, isDone: boolean): string {
  if (!isDone) {
    return `Reading the installed fonts: ${String(count.done)} of ${String(count.total)} families…`;
  }

  const read = `Installed fonts read: ${String(count.read)} families`;

  return count.leftOut.length === 0
    ? `${read}.`
    : `${read}; ${String(count.leftOut.length)} left out, their files could not be read (${count.leftOut.join(", ")}).`;
}

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
 * read leaves the font list; a line counts the families read, then says those left out.
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
  update();
}
