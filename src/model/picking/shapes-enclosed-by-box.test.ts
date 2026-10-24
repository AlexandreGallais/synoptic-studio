import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { HELLO_TEXT } from "../../../tests";
import { NO_HANDLES, boundingBox } from "../../geometry";
import { rectangleAsFreeShape } from "../rectangle";

import { shapePickArea } from "./shape-pick-area";
import { shapesEnclosedByBox } from "./shapes-enclosed-by-box";
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
 * (100, 0); its box 100 × 130 at (0, −30), the points stored in its frame.
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

/** A box on integers, at least 1 × 1. */
const BOX = fc.record({
  height: fc.integer({ max: 100, min: 1 }),
  width: fc.integer({ max: 100, min: 1 }),
  x: fc.integer({ max: 100, min: -100 }),
  y: fc.integer({ max: 100, min: -100 }),
});

describe("shapesEnclosedByBox (EN-144, DERIV-box-overlap)", () => {
  it("[F37.AC3] encloses A alone, B being held in half", () => {
    // (−10, −10) to (200, 110): A from 0 to 100 within it; B reaches x = 250.
    const areas = [shapePickArea(SHAPE_A), shapePickArea(SHAPE_B), shapePickArea(SHAPE_C)];

    expect(shapesEnclosedByBox(areas, dragged({ x: -10, y: -10 }, { x: 200, y: 110 }))).toEqual([
      0,
    ]);
  });

  it("[F37.AC3] encloses a circle by its four cubics' extremes", () => {
    // Its top, right, bottom and left points (350, 0), (400, 50), (350, 100), (300, 50) are its
    // extremes; [299, −1] to [401, 101] holds them, [301, −1] to [401, 101] leaves out the left.
    const areas = [shapePickArea(SHAPE_C)];

    expect(shapesEnclosedByBox(areas, dragged({ x: 299, y: -1 }, { x: 401, y: 101 }))).toEqual([0]);
    expect(shapesEnclosedByBox(areas, dragged({ x: 301, y: -1 }, { x: 401, y: 101 }))).toEqual([]);
  });

  it("[F37.AC3] encloses a free shape by its cubic side's top", () => {
    // The arch tops at (50, −30): [0, −30] to [100, 100] holds it, sides included;
    // [0, −29] to [100, 100] cuts it.
    const areas = [shapePickArea(ARCHED)];

    expect(shapesEnclosedByBox(areas, dragged({ x: 0, y: -30 }, { x: 100, y: 100 }))).toEqual([0]);
    expect(shapesEnclosedByBox(areas, dragged({ x: 0, y: -29 }, { x: 100, y: 100 }))).toEqual([]);
  });

  it("[F37.AC3] encloses a rounded rectangle by its sides, not its box's corners", () => {
    // Radius 20: [0, 100]² holds it, its contour box's left computed at about −3.6 × 10⁻¹⁵ from
    // the arcs, within EPSILON; [1, 100] × [0, 100] leaves out the left side x = 0.
    const areas = [shapePickArea(rectangle({ angle: 0, height: 100, width: 100, x: 0, y: 0 }, 20))];

    expect(shapesEnclosedByBox(areas, dragged({ x: 0, y: 0 }, { x: 100, y: 100 }))).toEqual([0]);
    expect(shapesEnclosedByBox(areas, dragged({ x: 1, y: 0 }, { x: 100, y: 100 }))).toEqual([]);
  });

  it("[F37.AC3] encloses a turned square by its turned corners", () => {
    // Turned 45° about (50, 50): its corners reach 50 ± 50√2 ≈ −20.71 and 120.71.
    const areas = [shapePickArea(rectangle({ angle: 45, height: 100, width: 100, x: 0, y: 0 }, 0))];

    expect(shapesEnclosedByBox(areas, dragged({ x: -21, y: -21 }, { x: 121, y: 121 }))).toEqual([
      0,
    ]);

    expect(shapesEnclosedByBox(areas, dragged({ x: -20, y: -21 }, { x: 121, y: 121 }))).toEqual([]);
  });

  it("[F37.AC3] encloses a text by its frame, and a shape 0 × 80 by its line", () => {
    // The frame from (10, 20) to (90, 60); the line x = 10 from y = 10 to 90.
    const text = [
      textPickArea(
        { ...HELLO_TEXT, angle: 0, isInBox: false, x: 10, y: 20 },
        { height: 40, width: 80 },
      ),
    ];
    const flat = [shapePickArea(rectangle({ angle: 0, height: 80, width: 0, x: 10, y: 10 }, 0))];

    expect(shapesEnclosedByBox(text, dragged({ x: 10, y: 20 }, { x: 90, y: 60 }))).toEqual([0]);
    expect(shapesEnclosedByBox(text, dragged({ x: 11, y: 20 }, { x: 90, y: 60 }))).toEqual([]);
    expect(shapesEnclosedByBox(flat, dragged({ x: 0, y: 0 }, { x: 20, y: 100 }))).toEqual([0]);
    expect(shapesEnclosedByBox(flat, dragged({ x: 0, y: 0 }, { x: 20, y: 50 }))).toEqual([]);
  });
});

describe("shapesEnclosedByBox (properties)", () => {
  test.prop({ boxes: fc.array(BOX, { maxLength: 5 }), rectangle: BOX })(
    "takes the upright boxes whose intervals lie within the rectangle's on both axes",
    ({ boxes, rectangle: drawn }) => {
      const asked = dragged(drawn, { x: drawn.x + drawn.width, y: drawn.y + drawn.height });
      const areas = boxes.map((each) => shapePickArea(rectangle({ ...each, angle: 0 }, 0)));
      const within = boxes.flatMap((each, index) =>
        each.x >= asked.minX &&
        each.x + each.width <= asked.maxX &&
        each.y >= asked.minY &&
        each.y + each.height <= asked.maxY
          ? [index]
          : [],
      );

      expect(shapesEnclosedByBox(areas, asked)).toEqual(within);
    },
  );
});
