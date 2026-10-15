import { isSameFamilyName } from "../../src";
import { selectElement, writeSelectOptions } from "../page";

import { readFontSession } from "./read-font-session";

import type { FontSession } from "./font-session";

/**
 * Reads the families the font list offers, each once: those kept, Inter first, then the installed
 * ones the browser listed and not read yet, their names matched without regard to case.
 *
 * @kind procedure
 * @param session - the session's fonts
 * @returns the families' names, in the list's order
 * @see docs/backlog/stories/E15-F15-US-036-fonts-embedding-and-html-layer.md
 */
function readFontListNames(session: FontSession): readonly string[] {
  const names = [
    ...session.kept.map((family) => family.name),
    ...session.installed.map((font) => font.family),
  ];

  return names.filter(
    (name, index) => names.findIndex((other) => isSameFamilyName(name, other)) === index,
  );
}

/**
 * Reads the family the font list chooses: the one asked when listed, else the first, Inter.
 *
 * @kind procedure
 * @param names - the families listed
 * @param family - the family asked
 * @returns the name of the family chosen, as listed
 * @see docs/backlog/stories/E15-F15-US-036-fonts-embedding-and-html-layer.md
 */
function readFontChoice(names: readonly string[], family: string): string {
  const [first = ""] = names;

  return names.find((name) => isSameFamilyName(name, family)) ?? first;
}

/**
 * Writes the font list: one option per family the editor can measure or read, Inter first (no
 * name typed, so no font without metrics can be chosen, US-036), the family asked chosen, else the
 * one chosen before, else Inter.
 *
 * @kind procedure
 * @param document - page document
 * @param family - the family to choose, matched without regard to case; empty to keep the one chosen
 * @see docs/backlog/stories/E15-F15-US-036-fonts-embedding-and-html-layer.md
 */
export function writeFontOptions(document: Document, family: string): void {
  const names = readFontListNames(readFontSession(document));
  const font = selectElement(document, "text-font");
  const chosen = font instanceof HTMLSelectElement ? font.value : "";

  writeSelectOptions(document, {
    id: "text-font",
    options: names.map((name) => ({ label: name, value: name })),
    value: readFontChoice(names, family === "" ? chosen : family),
  });
}
