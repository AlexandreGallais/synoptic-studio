import { selectElement } from "../page";

import type { PickedColor } from "../../src";

/**
 * Keeps the values the open picker holds on it, so that its hue stays the slider's while it is
 * open, even for a grey (ADR-0056, decision 3).
 *
 * @kind procedure
 * @param document - page document
 * @param picked - the hue, the saturation and the lightness or brightness
 * @see docs/backlog/stories/E15-F16-US-035-pick-a-color.md
 */
export function writePickedColor(document: Document, picked: PickedColor): void {
  const { dataset } = selectElement(document, "color-picker");

  dataset["hue"] = String(picked.hue);
  dataset["saturation"] = String(picked.saturation);
  dataset["level"] = String(picked.level);
}
