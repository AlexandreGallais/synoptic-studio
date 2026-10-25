import { writeWeightChoice } from "../fonts";
import { selectElement } from "../page";

import { readFieldValue } from "./read-field-value";

/** Weights of the bold button, pressed and released (`REF-CSS-FONTS-4`, §2.2: bold 700, normal 400). */
const WEIGHTS = { bold: 700, normal: 400 };

/**
 * Holds the weight before on the bold button when the weight chosen differs from it, so that a
 * family whose nearest face for 700 is the weight before holds nothing.
 *
 * @kind procedure
 * @param document - page document
 * @param weight - the weight before Bold was pressed
 * @see US-037
 */
function writeHeldWeight(document: Document, weight: string): void {
  if (readFieldValue(document, "text-weight") !== weight) {
    selectElement(document, "text-bold").dataset["heldWeight"] = weight;
  }
}

/**
 * Presses the bold button with no weight held: 400 at 700; otherwise 700, the weight before held.
 *
 * @kind procedure
 * @param document - page document
 * @param weight - the weight chosen
 * @see US-037
 */
function writeBoldWeight(document: Document, weight: string): void {
  if (Number(weight) === WEIGHTS.bold) {
    writeWeightChoice(document, WEIGHTS.normal);

    return;
  }

  writeWeightChoice(document, WEIGHTS.bold);
  writeHeldWeight(document, weight);
}

/**
 * Presses the bold button: a weight held is given back and forgotten; at 700 with no weight held,
 * 400; otherwise 700, the weight before held when the family's face for 700 differs from it.
 * Each weight is chosen among the faces of the family chosen, the nearest face when it lacks the
 * weight (`writeWeightChoice`, US-036): from 100, 700 then 100; on a family of 300 and 600, from
 * 300, 600 then 300; on a family of 400 alone, 400 kept and nothing held (US-037).
 *
 * @kind procedure
 * @param document - page document
 * @see US-037
 */
export function pressBold(document: Document): void {
  const bold = selectElement(document, "text-bold");
  const held = bold.dataset["heldWeight"];

  if (held === undefined) {
    writeBoldWeight(document, readFieldValue(document, "text-weight"));

    return;
  }

  delete bold.dataset["heldWeight"];
  writeWeightChoice(document, Number(held));
}
