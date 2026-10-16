import { boxAroundBlock } from "../../src";
import { writeInputValue } from "../page";
import { readDrawnText } from "../text-placement";

import { readFieldValue } from "./read-field-value";

import type { Size } from "../../src";

/**
 * Reads the block of the text last drawn when it fitted its content.
 *
 * @kind procedure
 * @param document - page document
 * @returns its measured block, nothing when it sat in a box or before a first text
 * @see docs/backlog/stories/E15-F17-US-055-fit-to-box-keeps-the-size.md
 */
function readFittedBlock(document: Document): Size | undefined {
  const drawn = readDrawnText(document);

  return drawn?.text.isInBox === false ? drawn.block : undefined;
}

/**
 * Writes in the box's width and height the block of a text that fitted its content, rounded up to
 * integers (`boxAroundBlock`, US-055), when Box is chosen: the text keeps its lines and its place,
 * and a turned text its corner, as any change of its frame's size (`choosePlacement`, ADR-0057).
 * The block is the one last drawn, measured again at each switch; a text already in a box keeps
 * the box typed.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E15-F17-US-055-fit-to-box-keeps-the-size.md
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
