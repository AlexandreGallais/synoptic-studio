import { isValidRoundShape, roundShapeCorners, turnedCorners } from "../src";

import { readAngle } from "./read-angle";
import { readNumber } from "./read-number";
import { readPosition } from "./read-position";

import type { ShapeReading } from "./shape-reading";
import type { RoundShape } from "../src";

/** Message shown when a value of the round shape is not allowed. */
const INVALID_ROUND = "Width and height must be integers ≥ 0, and the angle an integer.";

/**
 * Reads the round shape typed in the inputs (F10, ADR-0036): its width, height, angle and
 * position, with its corners as drawn when it is valid — a circle in a square box, a capsule
 * otherwise. It has no radius field: its corners' radius, half the smaller side, is shown.
 *
 * @kind procedure
 * @param document - page document
 * @returns the reading of the round shape
 * @see docs/backlog/stories/E01-F10-US-020-round-shape-playground.md
 */
export function readRoundShape(document: Document): ShapeReading {
  const model: RoundShape = {
    angle: readAngle(document),
    height: readNumber(document, "height"),
    kind: "round",
    width: readNumber(document, "width"),
    ...readPosition(document),
  };
  const isValid = isValidRoundShape(model);
  const corners = isValid ? turnedCorners(roundShapeCorners(model), model) : [];

  return {
    corners,
    isValid,
    message: INVALID_ROUND,
    model,
    radii: corners.map((corner) => corner.radius),
  };
}
