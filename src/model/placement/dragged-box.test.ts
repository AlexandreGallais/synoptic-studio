import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { draggedBox } from "./dragged-box";

/** A coordinate of the pointer in eighths of a unit, exact in a double, on a canvas of 5 000 pixels. */
const COORDINATE = fc.integer({ max: 40_000, min: -40_000 }).map((eighths) => eighths / 8);

/** A point of the pointer, possibly fractional. */
const POINT = fc.record({ x: COORDINATE, y: COORDINATE });

describe("draggedBox (F36)", () => {
  it("[F36.AC1] draws a box 120 × 80 at (10, 10) from (10, 10) to (130, 90)", () => {
    // 130 − 10 = 120 wide, 90 − 10 = 80 high, its top-left the press.
    expect(
      draggedBox({ isSquare: false, press: { x: 10, y: 10 }, release: { x: 130, y: 90 } }),
    ).toEqual({ height: 80, width: 120, x: 10, y: 10 });
  });

  it("[F36.AC1] draws the same box dragged up and to the left, from (130, 90) to (10, 10)", () => {
    // The box runs from the smaller corner, (10, 10), to the larger, (130, 90).
    expect(
      draggedBox({ isSquare: false, press: { x: 130, y: 90 }, release: { x: 10, y: 10 } }),
    ).toEqual({ height: 80, width: 120, x: 10, y: 10 });
  });

  it.each([
    // Up and right: x from the press, y from the release: (10, 10) to (130, 90) again.
    { press: { x: 10, y: 90 }, release: { x: 130, y: 10 } },
    // Down and left: x from the release, y from the press.
    { press: { x: 130, y: 10 }, release: { x: 10, y: 90 } },
  ])("[F36.AC1] draws the same box from $press.x, $press.y to $release.x, $release.y", (drag) => {
    expect(draggedBox({ ...drag, isSquare: false })).toEqual({
      height: 80,
      width: 120,
      x: 10,
      y: 10,
    });
  });

  it("[F36.AC1] puts the corners on integers, each rounded to the nearest, a tie up", () => {
    // (10.4, 9.6) rounds to (10, 10); (130.5, 90.49) to (131, 90): 121 × 80.
    expect(
      draggedBox({ isSquare: false, press: { x: 10.4, y: 9.6 }, release: { x: 130.5, y: 90.49 } }),
    ).toEqual({ height: 80, width: 121, x: 10, y: 10 });
  });

  it("[F36.AC1] rounds −0.5 to 0, never −0", () => {
    // (−0.5, −1.5) rounds up to (0, −1); (20, 20) is on integers: 20 × 21 at (0, −1).
    const box = draggedBox({
      isSquare: false,
      press: { x: -0.5, y: -1.5 },
      release: { x: 20, y: 20 },
    });

    expect(Object.is(box.x, 0)).toBe(true);
    expect(box).toEqual({ height: 21, width: 20, x: 0, y: -1 });
  });

  it("[F36.AC1] keeps the box square with Shift, its side the larger distance: a circle 120 × 120", () => {
    // 120 and 80 dragged: the side is 120, laid from the press down and right.
    expect(
      draggedBox({ isSquare: true, press: { x: 10, y: 10 }, release: { x: 130, y: 90 } }),
    ).toEqual({ height: 120, width: 120, x: 10, y: 10 });
  });

  it("[F36.AC1] lays a square from the press toward the release, up and to the left", () => {
    // From (130, 90) toward (10, 10): side 120, x = 130 − 120 = 10, y = 90 − 120 = −30.
    expect(
      draggedBox({ isSquare: true, press: { x: 130, y: 90 }, release: { x: 10, y: 10 } }),
    ).toEqual({ height: 120, width: 120, x: 10, y: -30 });
  });

  it("[F36.AC1] gives a box 0 × 0 at the press, a click, when released on its integer point", () => {
    // (10.2, 10.3) and (9.8, 10.4) both round to (10, 10).
    expect(
      draggedBox({ isSquare: false, press: { x: 10.2, y: 10.3 }, release: { x: 9.8, y: 10.4 } }),
    ).toEqual({ height: 0, width: 0, x: 10, y: 10 });

    expect(
      draggedBox({ isSquare: true, press: { x: 10, y: 10 }, release: { x: 10, y: 10 } }),
    ).toEqual({ height: 0, width: 0, x: 10, y: 10 });
  });

  it("[F36.AC1] keeps a width of 0 when dragged straight down, a height of 0 straight left", () => {
    // (10, 10) to (10.3, 90): 0 × 80; (130, 90) to (10, 90.2): 120 × 0 at (10, 90).
    expect(
      draggedBox({ isSquare: false, press: { x: 10, y: 10 }, release: { x: 10.3, y: 90 } }),
    ).toEqual({ height: 80, width: 0, x: 10, y: 10 });

    expect(
      draggedBox({ isSquare: false, press: { x: 130, y: 90 }, release: { x: 10, y: 90.2 } }),
    ).toEqual({ height: 0, width: 120, x: 10, y: 90 });
  });

  it("[F36.AC1] lays a square dragged straight left downward, level with the press", () => {
    // From (130, 90) to (10, 90): side 120, x = 130 − 120 = 10, y the press's 90.
    expect(
      draggedBox({ isSquare: true, press: { x: 130, y: 90 }, release: { x: 10, y: 90 } }),
    ).toEqual({ height: 120, width: 120, x: 10, y: 90 });
  });

  test.prop({ press: POINT, release: POINT })(
    "gives integers whose box has the rounded press and release at two opposite corners",
    ({ press, release }) => {
      const box = draggedBox({ isSquare: false, press, release });
      const xs = [Math.round(press.x) + 0, Math.round(release.x) + 0].toSorted((a, b) => a - b);
      const ys = [Math.round(press.y) + 0, Math.round(release.y) + 0].toSorted((a, b) => a - b);

      expect([box.x, box.x + box.width, box.y, box.y + box.height]).toEqual([...xs, ...ys]);
    },
  );

  test.prop({ press: POINT, release: POINT })(
    "draws the same box whichever end is pressed",
    ({ press, release }) => {
      expect(draggedBox({ isSquare: false, press: release, release: press })).toEqual(
        draggedBox({ isSquare: false, press, release }),
      );
    },
  );

  test.prop({ press: POINT, release: POINT })(
    "keeps a square's side the larger distance, one corner on the rounded press",
    ({ press, release }) => {
      const box = draggedBox({ isSquare: true, press, release });
      const free = draggedBox({ isSquare: false, press, release });
      const corners = [
        [box.x, box.y],
        [box.x + box.width, box.y],
        [box.x, box.y + box.height],
        [box.x + box.width, box.y + box.height],
      ];

      expect(box.width).toBe(Math.max(free.width, free.height));
      expect(box.height).toBe(box.width);
      expect(corners).toContainEqual([Math.round(press.x) + 0, Math.round(press.y) + 0]);
    },
  );
});
