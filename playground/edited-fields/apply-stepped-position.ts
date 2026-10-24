import { fieldArrowShift } from "../../src";
import { DRAWING_KEYS, readStoredDrawing } from "../kept-shapes";
import { readPanelValues } from "../picking";

import { applyEditedValue } from "./apply-edited-value";

/**
 * Moves the shape the panel edits alone by an arrow in its X or Y (F46): as the arrow on the
 * drawing along that axis, landed on integers as a drag's release (`fieldArrowShift`), from its
 * straight box as last drawn (`readPanelValues`), its X and Y then set to the landed values as
 * whole numbers typed (`applyEditedValue`); a shape refused, or a text never measured, which has
 * no straight box, takes the step as a value typed, as before.
 *
 * @kind procedure
 * @param document - page document
 * @param axis - the field the arrow is pressed in: `x` or `y`
 * @param step - the field's step: 1 or 10 for Up, −1 or −10 for Down
 * @see docs/domain/interaction.md#_2-selector
 */
export function applySteppedPosition(document: Document, axis: "x" | "y", step: number): void {
  const [kept] = readStoredDrawing(document, DRAWING_KEYS.edited);
  const [anchor] = kept === undefined ? [] : readPanelValues(document, kept.shape);

  if (anchor === undefined) {
    applyEditedValue(document, [axis, `shape-${axis}`], { isRelative: true, value: step });

    return;
  }

  const shift = fieldArrowShift(anchor, axis, step);

  // Each axis by its shift, relative: a fraction lands the result on an integer, an integer
  // keeps the position whole, as `+1` typed (`wholeValueWithTyped`).
  applyEditedValue(document, ["x", "shape-x"], { isRelative: true, value: shift.x });
  applyEditedValue(document, ["y", "shape-y"], { isRelative: true, value: shift.y });
}
