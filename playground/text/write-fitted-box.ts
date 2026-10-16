import { boxAroundBlock } from "../../src";
import { selectElement, writeInputValue } from "../page";
import { readDrawnText } from "../text-placement";

import { readFieldValue } from "./read-field-value";

import type { Size } from "../../src";

/**
 * Reads the block of the text last drawn when it fitted its content, unless the designer typed
 * the box's width or height (`#text-options`' `data-box-typed`, US-057).
 *
 * @kind procedure
 * @param document - page document
 * @returns its measured block; nothing when it sat in a box, its box was typed, or before a text
 * @see docs/backlog/stories/E15-F17-US-057-box-size-kept.md
 */
function readFittedBlock(document: Document): Size | undefined {
  const drawn = readDrawnText(document);
  const isTyped = selectElement(document, "text-options").dataset["boxTyped"] === "true";

  return !isTyped && drawn?.text.isInBox === false ? drawn.block : undefined;
}

/**
 * Writes in the box's width and height the block of a text that fitted its content, rounded up to
 * integers (`boxAroundBlock`, US-055), when Box is chosen: the text keeps its lines and its place,
 * and a turned text its corner, as any change of its frame's size (`choosePlacement`, ADR-0057).
 * The block is the one last drawn, measured again at each switch until the designer types the
 * box's width or height (`#text-options`' `data-box-typed`, `listenToTextFields`): from then on
 * the box typed comes back (US-057). A text already in a box keeps the box typed.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E15-F17-US-057-box-size-kept.md
 */
export function writeFittedBox(document: Document): void {
  const block =
    readFieldValue(document, "text-fit") === "box" ? readFittedBlock(document) : undefined;

  if (block === undefined) {
    return;
  }

  const box = boxAroundBlock(block);

  writeInputValue(document, "text-box-width", box.boxWidth);
  writeInputValue(document, "text-box-height", box.boxHeight);
}
