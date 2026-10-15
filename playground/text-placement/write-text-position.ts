import { selectElement, writeInputValue } from "../page";

import { choosePlacement } from "./choose-placement";
import { readDrawnText } from "./read-drawn-text";
import { readPositionFields } from "./read-position-fields";

import type { DrawnTextRecord } from "./drawn-text-record";
import type { SizedText, Text } from "../../src";

/**
 * Places a text whose block or box changed size (`choosePlacement`, US-038): a turned text's position
 * computed again is written in X and Y, at 5 decimals, so that its turned top-left corner stays
 * where it was drawn. The text drawn is then kept, with what X and Y show, for the next change
 * (`readTextPosition`).
 *
 * @kind procedure
 * @param document - page document
 * @param sized - a valid text, as read from the fields, and its measured block
 * @returns the text at the position it is drawn at
 * @see docs/backlog/stories/E15-F05-US-038-a-turned-text-grows-in-place.md
 */
export function writeTextPosition(document: Document, sized: SizedText): Text {
  const placed = choosePlacement(readDrawnText(document), sized);

  if (placed.x !== sized.text.x || placed.y !== sized.text.y) {
    writeInputValue(document, "shape-x", placed.x);
    writeInputValue(document, "shape-y", placed.y);
  }

  const record: DrawnTextRecord = {
    block: sized.block,
    fields: readPositionFields(document),
    text: placed,
  };

  selectElement(document, "text-options").dataset["drawnText"] = JSON.stringify(record);

  return placed;
}
