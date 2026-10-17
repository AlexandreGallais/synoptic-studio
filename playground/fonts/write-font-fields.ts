import { isSameFamilyName } from "../../src";
import { selectElement } from "../page";

import { readFontSession } from "./read-font-session";
import { writeFontOptions } from "./write-font-options";
import { writeFontSession } from "./write-font-session";
import { writeWeightChoice } from "./write-weight-choice";
import { WRITTEN_FONT_LISTS } from "./written-font-lists";

import type { FontSession } from "./font-session";
import type { FontFamily } from "../../src";

/** The weight the page opens with, read when the list holds none chosen (`REF-CSS-FONTS-4`, §2.2). */
const REGULAR = 400;

/**
 * Reads the kept family the text may be measured with, shown in the font list: any once the
 * installed fonts are read; while they are, none read ahead — its files not read again —, so that
 * no family read ahead is chosen before the reading ends (US-060), a family imported meanwhile
 * measured with at once.
 *
 * @kind procedure
 * @param session - the session's fonts
 * @param chosen - the family shown in the font list
 * @returns the kept family; undefined when none is kept under that name or it may not be measured with
 * @see docs/backlog/stories/E15-F17-US-060-chosen-font-file-held.md
 */
function readMeasurableFamily(session: FontSession, chosen: string): FontFamily | undefined {
  const isReadAhead =
    session.installed.length > 0 &&
    session.readAhead.some((read) => isSameFamilyName(read.family, chosen));

  return isReadAhead ? undefined : session.kept.find((kept) => isSameFamilyName(kept.name, chosen));
}

/**
 * Keeps the family chosen in the font list as the one the text is measured with, when its metrics
 * are read: an installed family chosen and not read yet leaves the family chosen before it, and
 * so does a family read ahead while the installed fonts are read (`readMeasurableFamily`, US-060).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E15-F15-US-036-import-choose-and-embed-a-font.md
 */
function writeMeasuredFamily(document: Document): void {
  const font = selectElement(document, "text-font");
  const chosen = font instanceof HTMLSelectElement ? font.value : "";
  const session = readFontSession(document);
  const family = readMeasurableFamily(session, chosen);

  if (family !== undefined) {
    writeFontSession(document, { ...session, measured: family.name });
  }
}

/**
 * Reads the weight chosen in the weight list.
 *
 * @kind procedure
 * @param document - page document
 * @returns its value; empty when the list holds none chosen
 * @see docs/backlog/stories/E15-F05-US-037-bold-gives-back-the-weight-and-no-sentence-case.md
 */
function readChosenWeight(document: Document): string {
  const weight = selectElement(document, "text-weight");

  return weight instanceof HTMLSelectElement ? weight.value : "";
}

/**
 * Writes the font list again only when the families it shows changed since it was written: the
 * families kept or the installed fonts of the session, each list replaced whole when it changes;
 * a value of the text changed alone, its size or opacity, leaves it as it is (US-048).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E15-F17-US-048-fluid-changes-held-keys.md
 */
function writeChangedFontOptions(document: Document): void {
  const session = readFontSession(document);
  const written = WRITTEN_FONT_LISTS.get(selectElement(document, "text-font"));

  if (written?.kept !== session.kept || written.installed !== session.installed) {
    writeFontOptions(document, "");
  }
}

/**
 * Brings the font and weight lists up to date with the session's fonts before the text is read:
 * the families the editor can measure, the one chosen kept and measured with once its metrics are
 * read (`writeMeasuredFamily`); the weights of its faces, the weight chosen kept when the family
 * has it, else the nearest face's (`writeWeightChoice`); a weight so rewritten forgets the weight
 * the bold button held, so that Bold never shows pressed over a weight it did not give (US-037).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E15-F15-US-036-import-choose-and-embed-a-font.md
 */
export function writeFontFields(document: Document): void {
  const chosen = readChosenWeight(document);

  writeChangedFontOptions(document);
  writeMeasuredFamily(document);
  writeWeightChoice(document, chosen === "" ? REGULAR : Number(chosen));

  if (readChosenWeight(document) !== chosen) {
    delete selectElement(document, "text-bold").dataset["heldWeight"];
  }
}
