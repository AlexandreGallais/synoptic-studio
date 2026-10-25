import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { HELLO_TEXT } from "../../../tests";
import { NO_HANDLES } from "../../geometry";
import { rectangleAsFreeShape } from "../rectangle";

import { PICK_TOLERANCE } from "./pick-tolerance";
import { shapeAtPoint } from "./shape-at-point";
import { shapePickArea } from "./shape-pick-area";
import { textPickArea } from "./text-pick-area";

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

/** Shape A of the criterion: a box from (0, 0) to (100, 100). */
const SHAPE_A = rectangle({ angle: 0, height: 100, width: 100, x: 0, y: 0 }, 0);

/** Shape B, drawn after A: a box from (50, 50) to (150, 150). */
const SHAPE_B = rectangle({ angle: 0, height: 100, width: 100, x: 50, y: 50 }, 0);

/** The tolerance at scale 1: the screen's pixels are user units. */
const TOLERANCE = PICK_TOLERANCE;

/** The pull of the arch's handles, 40 up. */
const PULL = { x: 0, y: -40 };

/**
 * A square 100 × 100 at (0, 0) whose top side is an arch: its first point's handle after and its
 * second point's handle before both (0, −40), so that the side runs from (0, 0) up to y = −30 and
 * down to (100, 0): y = −120 t (1 − t), x = 300 t² − 200 t³. Its box takes the arch in, 100 × 130
 * at (0, −30), the points stored in its frame so that they are drawn where they are written plus
 * (0, −30) (`DERIV-free-shape-stretch`).
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

/** A round shape in a box 100 × 100 at (0, 0): a circle of radius 50 about (50, 50). */
const CIRCLE = { angle: 0, height: 100, kind: "round", width: 100, x: 0, y: 0 } satisfies Shape;

/** A text at (10, 20), straight, fitting its content. */
const TEXT = { ...HELLO_TEXT, angle: 0, isInBox: false, x: 10, y: 20 };

/** Its measured block, 80 × 40. */
const BLOCK = { height: 40, width: 80 };

/** A box on integers, at least 1 × 1. */
const BOX = fc.record({
  height: fc.integer({ max: 100, min: 1 }),
  width: fc.integer({ max: 100, min: 1 }),
  x: fc.integer({ max: 100, min: -100 }),
  y: fc.integer({ max: 100, min: -100 }),
});

describe("shapeAtPoint (DERIV-painted-area)", () => {
  it("[F36.AC2] gives the top shape holding the point: B, then A, then none", () => {
    // (75, 75) is in both, B drawn last; (25, 25) in A only; (200, 200) in neither.
    const areas = [shapePickArea(SHAPE_A), shapePickArea(SHAPE_B)];

    expect(shapeAtPoint(areas, { x: 75, y: 75 }, TOLERANCE)).toBe(1);
    expect(shapeAtPoint(areas, { x: 25, y: 25 }, TOLERANCE)).toBe(0);
    expect(shapeAtPoint(areas, { x: 200, y: 200 }, TOLERANCE)).toBe(-1);
  });

  it("[F36.AC2] gives none for an empty drawing", () => {
    expect(shapeAtPoint([], { x: 0, y: 0 }, TOLERANCE)).toBe(-1);
  });

  it("[F36.AC2] leaves out the cut corner of a rectangle of radius 20", () => {
    // The top-left fillet is centred on (20, 20): (3, 3) is 17√2 ≈ 24.04 from it, over 20;
    // (8, 8) is 12√2 ≈ 16.97, under 20.
    const areas = [shapePickArea(rectangle({ angle: 0, height: 100, width: 100, x: 0, y: 0 }, 20))];

    expect(shapeAtPoint(areas, { x: 3, y: 3 }, TOLERANCE)).toBe(-1);
    expect(shapeAtPoint(areas, { x: 8, y: 8 }, TOLERANCE)).toBe(0);
  });

  it("[F36.AC2] finds a round shape inside its circle only", () => {
    // From (50, 50): (5, 5) is 45√2 ≈ 63.64, (14, 14) 36√2 ≈ 50.91, both over 50; (16, 16) is
    // 34√2 ≈ 48.08, inside, the four cubics within 0.03 of the circle (ADR-0048).
    const areas = [shapePickArea(CIRCLE)];

    expect(shapeAtPoint(areas, { x: 5, y: 5 }, TOLERANCE)).toBe(-1);
    expect(shapeAtPoint(areas, { x: 14, y: 14 }, TOLERANCE)).toBe(-1);
    expect(shapeAtPoint(areas, { x: 16, y: 16 }, TOLERANCE)).toBe(0);
    expect(shapeAtPoint(areas, { x: 50, y: 50 }, TOLERANCE)).toBe(0);
  });

  it("[F36.AC2] finds a free shape by its cubic side", () => {
    // On y = −15 the arch runs from x ≈ 5.80583 to 94.19417: (50, −15) and (8, −15) are under
    // it, (2, −15) beside it, (50, −35) above its top at −30.
    const areas = [shapePickArea(ARCHED)];

    expect(shapeAtPoint(areas, { x: 50, y: -15 }, TOLERANCE)).toBe(0);
    expect(shapeAtPoint(areas, { x: 8, y: -15 }, TOLERANCE)).toBe(0);
    expect(shapeAtPoint(areas, { x: 2, y: -15 }, TOLERANCE)).toBe(-1);
    expect(shapeAtPoint(areas, { x: 50, y: -35 }, TOLERANCE)).toBe(-1);
  });

  it("[F36.AC2] finds a turned square by its diamond, not its box", () => {
    // Turned 45° about (50, 50), its corners 50√2 ≈ 70.71 from it on the axes: inside when
    // |x − 50| + |y − 50| < 70.71. (5, 5): 90, outside; (50, −15): 65, inside.
    const areas = [shapePickArea(rectangle({ angle: 45, height: 100, width: 100, x: 0, y: 0 }, 0))];

    expect(shapeAtPoint(areas, { x: 5, y: 5 }, TOLERANCE)).toBe(-1);
    expect(shapeAtPoint(areas, { x: 50, y: -15 }, TOLERANCE)).toBe(0);
  });

  it("[F36.AC2] finds a text by its block", () => {
    // Block 80 × 40 at (10, 20): (50, 40) in it, (95, 40) right of it.
    const areas = [textPickArea(TEXT, BLOCK)];

    expect(shapeAtPoint(areas, { x: 50, y: 40 }, TOLERANCE)).toBe(0);
    expect(shapeAtPoint(areas, { x: 95, y: 40 }, TOLERANCE)).toBe(-1);
  });

  it("[F36.AC2] finds a text turned 90° by its turned block", () => {
    // About its middle (50, 40), the block 80 × 40 stands 40 × 80: x from 30 to 70, y from 0
    // to 80. (50, 5) was above the straight block; (15, 40) was in it.
    const areas = [textPickArea({ ...TEXT, angle: 90 }, BLOCK)];

    expect(shapeAtPoint(areas, { x: 50, y: 5 }, TOLERANCE)).toBe(0);
    expect(shapeAtPoint(areas, { x: 15, y: 40 }, TOLERANCE)).toBe(-1);
  });

  it("[F36.AC2] finds a shape 0 × 80 within the tolerance of its line", () => {
    // Its line x = 10 from y = 10 to 90: (12, 50) is 2 away, (15, 50) 5, at scale 1.
    const areas = [shapePickArea(rectangle({ angle: 0, height: 80, width: 0, x: 10, y: 10 }, 0))];

    expect(shapeAtPoint(areas, { x: 12, y: 50 }, TOLERANCE)).toBe(0);
    expect(shapeAtPoint(areas, { x: 15, y: 50 }, TOLERANCE)).toBe(-1);
  });

  it("[F36.AC2] finds a shape 0 × 80 turned 90° by its turned line", () => {
    // About (10, 50), (10, 10) turns to (50, 50) and (10, 90) to (−30, 50): (0, 52) is 2 away,
    // (0, 57) 7.
    const areas = [shapePickArea(rectangle({ angle: 90, height: 80, width: 0, x: 10, y: 10 }, 0))];

    expect(shapeAtPoint(areas, { x: 0, y: 52 }, TOLERANCE)).toBe(0);
    expect(shapeAtPoint(areas, { x: 0, y: 57 }, TOLERANCE)).toBe(-1);
  });

  it("[F36.AC2] reaches a shape without area within a tolerance divided by the view's scale", () => {
    // At scale 2, 4 screen pixels are 2 user units: (12, 50) is reached, (13, 50) is not.
    const areas = [shapePickArea(rectangle({ angle: 0, height: 80, width: 0, x: 10, y: 10 }, 0))];

    expect(shapeAtPoint(areas, { x: 12, y: 50 }, PICK_TOLERANCE / 2)).toBe(0);
    expect(shapeAtPoint(areas, { x: 13, y: 50 }, PICK_TOLERANCE / 2)).toBe(-1);
  });

  it("[F36.AC2] finds an empty text, its block 0 wide, within the tolerance", () => {
    // Its block 0 × 40 at (10, 20): (12, 30) is 2 from its line.
    const areas = [textPickArea(TEXT, { height: 40, width: 0 })];

    expect(shapeAtPoint(areas, { x: 12, y: 30 }, TOLERANCE)).toBe(0);
  });
});

describe("shapeAtPoint (properties)", () => {
  test.prop({
    boxes: fc.array(BOX, { maxLength: 5 }),
    x: fc.integer({ max: 200, min: -100 }),
    y: fc.integer({ max: 200, min: -100 }),
  })("gives the last box strictly holding a point between integers", ({ boxes, x, y }) => {
    const point = { x: x + 0.5, y: y + 0.5 };
    const areas = boxes.map((box) => shapePickArea(rectangle({ ...box, angle: 0 }, 0)));
    const holding = boxes.findLastIndex(
      (box) =>
        point.x > box.x &&
        point.x < box.x + box.width &&
        point.y > box.y &&
        point.y < box.y + box.height,
    );

    expect(shapeAtPoint(areas, point, TOLERANCE)).toBe(holding);
  });
});
