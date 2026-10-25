import { selectElement, writeInputValue } from "../page";
import { writeNewTextFields } from "../placing";
import { writeShapePosition } from "../position";

import type { Point, Size } from "editor";

/**
 * Writes the box of a new shape drawn by a drag: a rectangle's, a polygon's or a round
 * shape's width, height and position; a new text (`writeNewTextFields`) in a box of that size,
 * the box kept as typed, its top-left the box's.
 *
 * @kind procedure
 * @param document - page document
 * @param tool - the tool armed: `rectangle`, `polygon`, `round` or `text`
 * @param box - the box dragged, on integers (`draggedBox`)
 */
export function writeDraggedShape(document: Document, tool: string, box: Point & Size): void {
  if (tool !== "text") {
    writeInputValue(document, "width", box.width);
    writeInputValue(document, "height", box.height);
    writeShapePosition(document, box);

    return;
  }

  writeNewTextFields(document);
  Object.assign(selectElement(document, "text-fit"), { value: "box" });
  selectElement(document, "text-options").dataset["boxTyped"] = "true";
  writeInputValue(document, "text-box-width", box.width);
  writeInputValue(document, "text-box-height", box.height);
  writeInputValue(document, "shape-x", box.x);
  writeInputValue(document, "shape-y", box.y);
}
