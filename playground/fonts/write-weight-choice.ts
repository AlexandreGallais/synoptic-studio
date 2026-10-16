import { isSameFamilyName, matchingFace } from "../../src";
import { selectElement, writeSelectOptions } from "../page";

import { readFontSession } from "./read-font-session";

import type { FontFamily } from "../../src";

/** The names of the common weights, as the weight list shows them (`REF-CSS-FONTS-4`, §2.2). */
const WEIGHT_NAMES: Readonly<Record<number, string>> = {
  100: "Thin",
  200: "Extra light",
  300: "Light",
  400: "Regular",
  500: "Medium",
  600: "Semi-bold",
  700: "Bold",
  800: "Extra bold",
  900: "Black",
};

/**
 * Reads the family chosen in the font list among the session's families.
 *
 * @kind procedure
 * @param document - page document
 * @returns the family; undefined for an installed family whose files are not read yet
 * @see docs/backlog/stories/E15-F15-US-036-fonts-embedding-and-html-layer.md
 */
function readChosenFamily(document: Document): FontFamily | undefined {
  const font = selectElement(document, "text-font");
  const chosen = font instanceof HTMLSelectElement ? font.value : "";

  return readFontSession(document).kept.find((kept) => isSameFamilyName(kept.name, chosen));
}

/**
 * Reads the label of a weight: its number, then its name when it has one (`450` alone).
 *
 * @kind procedure
 * @param weight - a face's weight
 * @returns such as `700 Bold`
 * @see docs/backlog/stories/E15-F15-US-036-fonts-embedding-and-html-layer.md
 */
function readWeightLabel(weight: number): string {
  const name = WEIGHT_NAMES[weight];

  return name === undefined ? String(weight) : `${String(weight)} ${name}`;
}

/**
 * Reads the weight of the family's face that matches a weight and the italic button, as CSS
 * chooses a face (`matchingFace`).
 *
 * @kind procedure
 * @param document - page document
 * @param family - the family chosen, with at least one face
 * @param weight - the weight asked
 * @returns the face's weight; the weight asked for a family without faces
 * @see docs/domain/shapes.md#text
 */
function readMatchingWeight(document: Document, family: FontFamily, weight: number): number {
  const isItalic = selectElement(document, "text-italic").getAttribute("aria-pressed") === "true";

  return matchingFace(family.faces, { isItalic, weight })?.weight ?? weight;
}

/**
 * Writes the weight list of the family chosen in the font list, its faces' weights only, from the
 * lightest, and chooses the weight of the face that matches the weight asked and the italic
 * button (`readMatchingWeight`, `docs/domain/shapes.md`, Font face): a family of 400 and 700
 * gives 700 for 700 and 400 for 300. A family whose files are not read yet, or without faces —
 * Inter when the browser draws none of its faces (US-053) —, keeps the list as it is.
 *
 * @kind procedure
 * @param document - page document
 * @param weight - the weight asked, from 100 to 900
 * @see docs/backlog/stories/E15-F15-US-036-fonts-embedding-and-html-layer.md
 */
export function writeWeightChoice(document: Document, weight: number): void {
  const family = readChosenFamily(document);

  if (family === undefined || family.faces.length === 0) {
    return;
  }

  const weights = [...new Set(family.faces.map((face) => face.weight))].toSorted((a, b) => a - b);

  writeSelectOptions(document, {
    id: "text-weight",
    options: weights.map((value) => ({ label: readWeightLabel(value), value: String(value) })),
    value: String(readMatchingWeight(document, family, weight)),
  });
}
