import { textPositionAfterResize } from "../../src";

import type { DrawnTextRecord } from "./drawn-text-record";
import type { SizedText, Text } from "../../src";

/** What a text keeps when it changes size: its angle and its position. */
const PLACEMENT_KEYS = ["angle", "x", "y"] as const;

/**
 * Chooses where a text is drawn (US-038, ADR-0057): when its angle and its position are those of
 * the text last drawn, at its position computed again so that its turned top-left corner stays
 * where it was drawn (`textPositionAfterResize`); a position typed, an angle changed, or a first
 * text, where it is.
 *
 * @kind procedure
 * @param drawn - the text last drawn, nothing before the first
 * @param sized - the text read from the fields, and its measured block
 * @returns the text at the position it is drawn at
 * @see docs/backlog/stories/E15-F05-US-038-a-turned-text-grows-in-place.md
 */
export function choosePlacement(drawn: DrawnTextRecord | undefined, sized: SizedText): Text {
  if (drawn === undefined) {
    return sized.text;
  }

  const isSamePlacement = PLACEMENT_KEYS.every((key) => drawn.text[key] === sized.text[key]);

  return isSamePlacement ? { ...sized.text, ...textPositionAfterResize(drawn, sized) } : sized.text;
}
