import { distinctFamilyNames, isSameFamilyName } from "../../src";
import { selectElement, writeSelectOptions } from "../page";

import { readFontSession } from "./read-font-session";
import { WRITTEN_FONT_LISTS } from "./written-font-lists";

import type { FontSession } from "./font-session";

/**
 * Reads the families the font list offers, each once: those kept, read and approved, Inter first
 * when the browser draws it, their names matched without regard to case; an installed family the
 * browser listed and not read yet is never offered, so that a family later refused never shows
 * (US-059).
 *
 * @kind procedure
 * @param session - the session's fonts
 * @returns the families' names, in the list's order
 */
function readFontListNames(session: FontSession): readonly string[] {
  return distinctFamilyNames(session.kept.map((family) => family.name));
}

/**
 * Reads the family the font list chooses: the one asked when listed; else the family the text is
 * measured with, so that a family left out of the list while it is chosen gives the list
 * back to it; else the first, Inter.
 *
 * @kind procedure
 * @param names - the families listed
 * @param family - the family asked
 * @param measured - the family the text is measured with
 * @returns the name of the family chosen, as listed
 */
function readFontChoice(names: readonly string[], family: string, measured: string): string {
  const [first = ""] = names;
  const asked = names.find((name) => isSameFamilyName(name, family));

  return asked ?? names.find((name) => isSameFamilyName(name, measured)) ?? first;
}

/**
 * Writes the font list: one option per family the editor can measure, Inter first (no
 * name typed, so no font without metrics can be chosen), the family asked chosen, else the
 * one chosen before, else Inter.
 *
 * @kind procedure
 * @param document - page document
 * @param family - the family to choose, matched without regard to case; empty to keep the one chosen
 */
export function writeFontOptions(document: Document, family: string): void {
  const session = readFontSession(document);
  const names = readFontListNames(session);
  const font = selectElement(document, "text-font");
  const chosen = font instanceof HTMLSelectElement ? font.value : "";

  writeSelectOptions(document, {
    id: "text-font",
    options: names.map((name) => ({ label: name, value: name })),
    value: readFontChoice(names, family === "" ? chosen : family, session.measured),
  });

  WRITTEN_FONT_LISTS.set(font, session);
}
