import { createPathElement, isRegularPolygon } from "../../src";

import { createFrameElements } from "./create-frame-elements";
import { createStraightBoxElement } from "./create-straight-box-element";
import { readBoxPathData } from "./read-box-path-data";

import type { BoundingBox, Shape } from "../../src";

/**
 * Draws the boxes of a shape behind it. A polygon's box, grey, shows its anchor (F07) and turns
 * with it. A turned shape also gets its straight box, dashed, and its turned box as a thin frame,
 * the selection (F04): drawn by fills, never by a native stroke.
 *
 * @kind procedure
 * @param document - page document
 * @param model - shape read from the inputs
 * @param straight - straight box of the drawn shape
 * @returns the polygon's box and, for a turned shape, its straight box and its frame
 * @see US-018
 */
export function createBoxElements(
  document: Document,
  model: Shape,
  straight: BoundingBox,
): readonly SVGPathElement[] {
  const box = createPathElement(document, readBoxPathData(model, 0));

  box.classList.add("box");

  const boxes = isRegularPolygon(model) ? [box] : [];

  return model.angle === 0
    ? boxes
    : [
        createStraightBoxElement(document, straight),
        ...boxes,
        ...createFrameElements(document, model, "turned-box"),
      ];
}
