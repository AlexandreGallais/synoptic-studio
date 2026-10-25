import { selectElement, writeInputValue } from "../page";

import { choosePlacement } from "./choose-placement";
import { readDrawnText } from "./read-drawn-text";
import { readPositionFields } from "./read-position-fields";
import { readStraightStart } from "./read-straight-start";

import type { DrawnTextRecord } from "./drawn-text-record";
import type { SizedText, Text } from "../../src";

/**
 * Places a text from the start of its straight box X and Y show (`choosePlacement`), then writes
 * in X and Y where its straight box starts as drawn, at 5 decimals (`readStraightStart`), as a
 * shape's (Product Owner, 2026-10-23, AUD-040): a turn or a growing turned text changing them, a
 * text at 0° its position. The text drawn is then kept, with what X and Y show, for the next
 * change (`readTextPosition`).
 *
 * @kind procedure
 * @param document - page document
 * @param sized - a valid text, as read from the fields, its position the start X and Y show, and
 * its measured block
 * @returns the text at the position it is drawn at
 * @see RULE-shapes-scripts-and-turning
 */
export function writeTextPosition(document: Document, sized: SizedText): Text {
  const placed = choosePlacement(readDrawnText(document), sized);
  const start = readStraightStart({ block: sized.block, text: placed });

  writeInputValue(document, "shape-x", start.x);
  writeInputValue(document, "shape-y", start.y);

  const record: DrawnTextRecord = {
    block: sized.block,
    fields: readPositionFields(document),
    text: placed,
  };

  selectElement(document, "text-options").dataset["drawnText"] = JSON.stringify(record);

  return placed;
}
