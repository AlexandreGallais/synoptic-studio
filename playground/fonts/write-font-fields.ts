import { selectElement } from "../page";

import { writeFontOptions } from "./write-font-options";
import { writeWeightChoice } from "./write-weight-choice";

/** The weight the page opens with, read when the list holds none chosen (`REF-CSS-FONTS-4`, §2.2). */
const REGULAR = 400;

/**
 * Brings the font and weight lists up to date with the session's fonts before the text is read:
 * the families the editor can measure, the one chosen kept; the weights of its faces, the weight
 * chosen kept when the family has it, else the nearest face's (`writeWeightChoice`).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E15-F15-US-036-fonts-embedding-and-html-layer.md
 */
export function writeFontFields(document: Document): void {
  const weight = selectElement(document, "text-weight");
  const chosen = weight instanceof HTMLSelectElement ? weight.value : "";

  writeFontOptions(document, "");
  writeWeightChoice(document, chosen === "" ? REGULAR : Number(chosen));
}
