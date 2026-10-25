import type { ColorModel } from "../../src";

/**
 * Reads the model the picker shows its values in: HSB, as the page opens, or HSL.
 *
 * @kind procedure
 * @param document - page document
 * @returns `hsl` when HSL is chosen, else `hsb`
 * @see US-035
 */
export function readColorModel(document: Document): ColorModel {
  return document.querySelector<HTMLSelectElement>("#color-model")?.value === "hsl" ? "hsl" : "hsb";
}
