import { ALIGNMENTS } from "../src";

import type { Alignment, Anchor } from "../src";

/**
 * Reads one alignment written in the value of an anchor button, `mid` for any other text.
 *
 * @kind procedure
 * @param text - part of the button value, `min`, `mid` or `max` when the page is right
 * @returns `min`, `mid` or `max`
 * @see docs/backlog/stories/E01-F07-US-010-anchor-picker-playground.md
 */
function readAlignment(text: string): Alignment {
  return ALIGNMENTS.find((alignment) => alignment === text) ?? "mid";
}

/**
 * Reads the anchor chosen in the 3 × 3 grid: the value of the checked button, horizontal then
 * vertical alignment (`max min` is the top right).
 *
 * @kind procedure
 * @param document - page document
 * @returns the anchor, centered when no button is checked
 * @see docs/backlog/stories/E01-F07-US-010-anchor-picker-playground.md
 */
export function readAnchor(document: Document): Anchor {
  const buttons = document.querySelectorAll<HTMLInputElement>('input[name="anchor"]');
  const checked = [...buttons].find((button) => button.checked);
  const [horizontal = "mid", vertical = "mid"] = (checked?.value ?? "").split(" ", 2);

  return { horizontal: readAlignment(horizontal), vertical: readAlignment(vertical) };
}
