import { selectElement, writeInputValue } from "../page";

import { choosePlacement } from "./choose-placement";
import { readDrawnText } from "./read-drawn-text";
import { readPositionFields } from "./read-position-fields";
import { readTurnedCorner } from "./read-turned-corner";

import type { DrawnTextRecord } from "./drawn-text-record";
import type { SizedText, Text } from "../../src";

/**
 * Places a text from the corner X and Y show (`choosePlacement`, US-054), then writes in X and Y
 * its turned top-left corner as drawn, at 5 decimals (`readTurnedCorner`): a turned text shows
 * where its top-left is drawn, a turn changing them, a text at 0° its position. The text drawn
 * is then kept, with what X and Y show, for the next change (`readTextPosition`).
 *
 * @kind procedure
 * @param document - page document
 * @param sized - a valid text, as read from the fields, its position the corner X and Y show,
 * and its measured block
 * @returns the text at the position it is drawn at
 * @see docs/backlog/stories/E15-F17-US-054-turned-text-position-shows-its-corner.md
 */
export function writeTextPosition(document: Document, sized: SizedText): Text {
  const placed = choosePlacement(readDrawnText(document), sized);
  const corner = readTurnedCorner({ block: sized.block, text: placed });

  writeInputValue(document, "shape-x", corner.x);
  writeInputValue(document, "shape-y", corner.y);

  const record: DrawnTextRecord = {
    block: sized.block,
    fields: readPositionFields(document),
    text: placed,
  };

  selectElement(document, "text-options").dataset["drawnText"] = JSON.stringify(record);

  return placed;
}
