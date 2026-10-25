import { selectElement } from "../page";

import type { PickedColor } from "../../src";

/**
 * Keeps the values the open picker holds on it, so that its hue stays the slider's while it is
 * open, even for a grey (ADR-0056, decision 3), and the color they were read from: each value
 * edited afterwards is rebuilt against its exact values, a value equal to the one read standing
 * for its exact one, a value typed kept as typed (ADR-0058, decisions 2 and 3).
 *
 * @kind procedure
 * @param document - page document
 * @param picked - the hue, the saturation and the lightness or brightness, integers
 * @param readColor - the color the values were read from, when the picker opened, a hexadecimal
 * was typed or another model chosen
 * @see US-040
 */
export function writePickedColor(document: Document, picked: PickedColor, readColor: string): void {
  const { dataset } = selectElement(document, "color-picker");

  dataset["hue"] = String(picked.hue);
  dataset["saturation"] = String(picked.saturation);
  dataset["level"] = String(picked.level);
  dataset["readColor"] = readColor;
}
