import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { HELLO_TEXT } from "../../../tests";
import { NO_HANDLES, boundingBox } from "../../geometry";
import { rectangleAsFreeShape } from "../rectangle";

import { shapePickArea } from "./shape-pick-area";
import { shapesTouchedByBox } from "./shapes-touched-by-box";
import { textPickArea } from "./text-pick-area";

import type { BoundingBox } from "../../geometry";
import type { Point } from "../../math";
import type { Rectangle } from "../rectangle";
import type { Shape } from "../shape";

/**
 * A rectangle of the rectangle tool, its four corners of one radius.
 *
 * @param box - its angle, position, width and height
 * @param radius - the radius of its four corners
 * @returns the free shape it makes
 */
function rectangle(
  box: Omit<Rectangle, "radii">,
  radius: number,
): ReturnType<typeof rectangleAsFreeShape> {
  return rectangleAsFreeShape({ ...box, radii: [radius, radius, radius, radius] });
}

/**
 * The box of a rectangle dragged from one point to another.
 *
 * @param from - the point pressed
 * @param to - the point released
 * @returns the box between them
 */
function dragged(from: Point, to: Point): BoundingBox {
  return boundingBox([from, to]);
}

/** Shape A of the criterion: a box from (0, 0) to (100, 100). */
const SHAPE_A = rectangle({ angle: 0, height: 100, width: 100, x: 0, y: 0 }, 0);

/** Shape B: a box from (150, 0) to (250, 100). */
const SHAPE_B = rectangle({ angle: 0, height: 100, width: 100, x: 150, y: 0 }, 0);

/** Shape C: a circle of radius 50 about (350, 50), in the box from (300, 0) to (400, 100). */
const SHAPE_C = { angle: 0, height: 100, kind: "round", width: 100, x: 300, y: 0 } satisfies Shape;

/** The pull of the arch's handles, 40 up. */
const PULL = { x: 0, y: -40 };

/**
 * A square 100 × 100 at (0, 0) whose top side is an arch from (0, 0) up to y = −30 and down to
 * (100, 0): y = −120 t (1 − t), x = 300 t² − 200 t³; its box 100 × 130 at (0, −30), the points
 * stored in its frame.
 */
const ARCHED = {
  angle: 0,
  corners: [
    { handles: { ...NO_HANDLES, after: PULL, isOn: true }, point: { x: 0, y: 30 }, radius: 0 },
    { handles: { ...NO_HANDLES, before: PULL, isOn: true }, point: { x: 100, y: 30 }, radius: 0 },
    { handles: NO_HANDLES, point: { x: 100, y: 130 }, radius: 0 },
    { handles: NO_HANDLES, point: { x: 0, y: 130 }, radius: 0 },
  ],
  height: 130,
  width: 100,
  x: 0,
  y: -30,
};

/** A text at (10, 20), straight, fitting its content. */
const TEXT = { ...HELLO_TEXT, angle: 0, isInBox: false, x: 10, y: 20 };

/** Its measured block, 80 × 40: its frame from (10, 20) to (90, 60). */
const BLOCK = { height: 40, width: 80 };

/** A box on integers, at least 1 × 1. */
const BOX = fc.record({
  height: fc.integer({ max: 100, min: 1 }),
  width: fc.integer({ max: 100, min: 1 }),
  x: fc.integer({ max: 100, min: -100 }),
  y: fc.integer({ max: 100, min: -100 }),
});

describe("shapesTouchedByBox (DERIV-box-overlap)", () => {
  it("[F37.AC3] touches A and B, which the rectangle holds whole and in half", () => {
    // (−10, −10) to (200, 110): A within it, B from x = 150 to 250 crossed by x = 200, C beyond.
    const areas = [shapePickArea(SHAPE_A), shapePickArea(SHAPE_B), shapePickArea(SHAPE_C)];

    expect(shapesTouchedByBox(areas, dragged({ x: -10, y: -10 }, { x: 200, y: 110 }))).toEqual([
      0, 1,
    ]);
  });

  it("[F37.AC3] leaves out a circle whose box only is covered", () => {
    // (390, 90) to (420, 120): its nearest point to (350, 50) is (390, 90), √(40² + 40²) ≈ 56.6
    // away, beyond the radius 50, the four cubics within 0.03 of the circle (ADR-0048).
    const areas = [shapePickArea(SHAPE_A), shapePickArea(SHAPE_B), shapePickArea(SHAPE_C)];

    expect(shapesTouchedByBox(areas, dragged({ x: 390, y: 90 }, { x: 420, y: 120 }))).toEqual([]);
  });

  it("[F37.AC3] takes a free shape by its cubic side, not its box", () => {
    // [0, 10] × [−30, −20], the corner of its box: on y = −20 the arch is at x ≈ 11.51, on
    // x = 10 at y ≈ −18.87, and (0, −30) is above it. [45, 55] × [−35, −25] holds its top
    // (50, −30). [−1, 101] × [−31, 101] holds it whole.
    const areas = [shapePickArea(ARCHED)];

    expect(shapesTouchedByBox(areas, dragged({ x: 0, y: -30 }, { x: 10, y: -20 }))).toEqual([]);
    expect(shapesTouchedByBox(areas, dragged({ x: 45, y: -35 }, { x: 55, y: -25 }))).toEqual([0]);
    expect(shapesTouchedByBox(areas, dragged({ x: -1, y: -31 }, { x: 101, y: 101 }))).toEqual([0]);
  });

  it("[F37.AC3] takes a rounded rectangle by its fillets, not its box", () => {
    // Radius 20, the top-left fillet about (20, 20): (5, 5) is 15√2 ≈ 21.21 from it, beyond 20;
    // (8, 8) is 12√2 ≈ 16.97, within.
    const areas = [shapePickArea(rectangle({ angle: 0, height: 100, width: 100, x: 0, y: 0 }, 20))];

    expect(shapesTouchedByBox(areas, dragged({ x: 0, y: 0 }, { x: 5, y: 5 }))).toEqual([]);
    expect(shapesTouchedByBox(areas, dragged({ x: 0, y: 0 }, { x: 8, y: 8 }))).toEqual([0]);
    expect(shapesTouchedByBox(areas, dragged({ x: 0, y: 0 }, { x: 100, y: 100 }))).toEqual([0]);
  });

  it("[F37.AC3] takes a turned square by its diamond, not its box", () => {
    // Turned 45° about (50, 50), its corners 50√2 ≈ 70.71 from it on the axes: its top-left
    // side is x + y = 29.29. [−20, 0]² reaches x + y = 0, outside; (20, 20) gives 40, inside.
    const areas = [shapePickArea(rectangle({ angle: 45, height: 100, width: 100, x: 0, y: 0 }, 0))];

    expect(shapesTouchedByBox(areas, dragged({ x: -20, y: -20 }, { x: 0, y: 0 }))).toEqual([]);
    expect(shapesTouchedByBox(areas, dragged({ x: 0, y: 0 }, { x: 20, y: 20 }))).toEqual([0]);
  });

  it("[F37.AC3] takes a text by its frame", () => {
    // Its frame from (10, 20) to (90, 60): [85, 95] × [50, 70] crosses it, [91, 100] × [0, 100]
    // lies right of it.
    const areas = [textPickArea(TEXT, BLOCK)];

    expect(shapesTouchedByBox(areas, dragged({ x: 85, y: 50 }, { x: 95, y: 70 }))).toEqual([0]);
    expect(shapesTouchedByBox(areas, dragged({ x: 91, y: 0 }, { x: 100, y: 100 }))).toEqual([]);
  });

  it("[F37.AC3] takes a shape 0 × 80 by its line, with no tolerance", () => {
    // Its line x = 10 from y = 10 to 90: [5, 15] × [40, 50] crosses it; [12, 20] × [0, 100] is 2
    // from it, within the 4 pixels of a click but not touching.
    const areas = [shapePickArea(rectangle({ angle: 0, height: 80, width: 0, x: 10, y: 10 }, 0))];

    expect(shapesTouchedByBox(areas, dragged({ x: 5, y: 40 }, { x: 15, y: 50 }))).toEqual([0]);
    expect(shapesTouchedByBox(areas, dragged({ x: 12, y: 0 }, { x: 20, y: 100 }))).toEqual([]);
  });

  it("[F37.AC3] takes a polygon along a side its contour computes with a rounding error", () => {
    // The Polygon tool's square, 4 corners of radius 10, 100 × 100 at (0, 0): its left side
    // lands at x ≈ 1.1 × 10⁻¹⁵, the cosines of its corners' angles not exact; [−10, 0] × [40, 60]
    // reaches x = 0, within EPSILON of it, as [0, 100]² encloses it. The stretched decagon's
    // bottom side lands at y ≈ 100 − 1.4 × 10⁻¹⁴; [0, 100] × [100, 110] reaches it so. [−10, −1]
    // × [40, 60] stays 1 away from the square.
    const polygon = {
      anchor: { horizontal: "mid", vertical: "mid" },
      angle: 0,
      height: 100,
      stretched: true,
      width: 100,
      x: 0,
      y: 0,
    } as const;
    const square = shapePickArea({ ...polygon, corners: 4, radii: [10, 10, 10, 10] });
    const decagon = shapePickArea({
      ...polygon,
      corners: 10,
      radii: Array.from({ length: 10 }, () => 0),
    });

    expect(shapesTouchedByBox([square], dragged({ x: -10, y: 40 }, { x: 0, y: 60 }))).toEqual([0]);
    expect(shapesTouchedByBox([square], dragged({ x: -10, y: 40 }, { x: -1, y: 60 }))).toEqual([]);

    expect(shapesTouchedByBox([decagon], dragged({ x: 0, y: 100 }, { x: 100, y: 110 }))).toEqual([
      0,
    ]);
  });

  it("[F37.AC3] takes none from an empty drawing", () => {
    expect(shapesTouchedByBox([], dragged({ x: 0, y: 0 }, { x: 100, y: 100 }))).toEqual([]);
  });
});

describe("shapesTouchedByBox (properties)", () => {
  test.prop({ boxes: fc.array(BOX, { maxLength: 5 }), rectangle: BOX })(
    "takes the upright boxes whose intervals overlap the rectangle's on both axes",
    ({ boxes, rectangle: drawn }) => {
      const asked = dragged(drawn, { x: drawn.x + drawn.width, y: drawn.y + drawn.height });
      const areas = boxes.map((each) => shapePickArea(rectangle({ ...each, angle: 0 }, 0)));
      const overlapping = boxes.flatMap((each, index) =>
        each.x <= asked.maxX &&
        each.x + each.width >= asked.minX &&
        each.y <= asked.maxY &&
        each.y + each.height >= asked.minY
          ? [index]
          : [],
      );

      expect(shapesTouchedByBox(areas, asked)).toEqual(overlapping);
    },
  );
});
