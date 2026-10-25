import { isDrawnText } from "../../src";

import { readModel } from "./read-model";

import type { Shape } from "../../src";

/**
 * Model of the shape the panel edits (`readModel`), for a test that edits a shape, not a text.
 *
 * @kind procedure
 * @returns the model, a shape
 * @throws {TypeError} when the panel edits a text
 */
export function readShapeModel(): Shape {
  const model = readModel();

  if (isDrawnText(model)) {
    throw new TypeError("The panel edits a text, not a shape.");
  }

  return model;
}
