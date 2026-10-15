import { isSameFamilyName } from "../../src";
import { selectElement } from "../page";

import { readFontSession } from "./read-font-session";
import { writeFontOptions } from "./write-font-options";
import { writeFontSession } from "./write-font-session";
import { writeWeightChoice } from "./write-weight-choice";

/** The weight the page opens with, read when the list holds none chosen (`REF-CSS-FONTS-4`, §2.2). */
const REGULAR = 400;

/**
 * Keeps the family chosen in the font list as the one the text is measured with, when its metrics
 * are read: an installed family chosen and not read yet leaves the family chosen before it.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E15-F15-US-036-fonts-embedding-and-html-layer.md
 */
function writeMeasuredFamily(document: Document): void {
  const font = selectElement(document, "text-font");
  const chosen = font instanceof HTMLSelectElement ? font.value : "";
  const session = readFontSession(document);
  const family = session.kept.find((kept) => isSameFamilyName(kept.name, chosen));

  if (family !== undefined) {
    writeFontSession(document, { ...session, measured: family.name });
  }
}

/**
 * Brings the font and weight lists up to date with the session's fonts before the text is read:
 * the families the editor can measure, the one chosen kept and measured with once its metrics are
 * read (`writeMeasuredFamily`); the weights of its faces, the weight chosen kept when the family
 * has it, else the nearest face's (`writeWeightChoice`).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E15-F15-US-036-fonts-embedding-and-html-layer.md
 */
export function writeFontFields(document: Document): void {
  const weight = selectElement(document, "text-weight");
  const chosen = weight instanceof HTMLSelectElement ? weight.value : "";

  writeFontOptions(document, "");
  writeMeasuredFamily(document);
  writeWeightChoice(document, chosen === "" ? REGULAR : Number(chosen));
}
