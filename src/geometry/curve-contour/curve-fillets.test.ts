import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { bentRectangle, rectangleCorners } from "../../../tests";
import { filletArcs } from "../corner-radius";

import { curveFillets } from "./curve-fillets";

import type { Arc } from "../contour";
import type { CurveCorner } from "../handles";

/** An arc without length at the origin, read when a corner is missing. */
const NO_ARC: Arc = {
  center: { x: 0, y: 0 },
  end: { x: 0, y: 0 },
  kind: "arc",
  positive: false,
  radius: 0,
  start: { x: 0, y: 0 },
};

/**
 * The arc drawn at one corner.
 *
 * @param corners - corners of the shape
 * @param index - the corner's number, from 0
 * @returns its fillet's arc
 */
function arcAt(corners: readonly CurveCorner[], index: number): Arc {
  return curveFillets(corners)[index]?.arc ?? NO_ARC;
}

describe("curveFillets (ADR-0047, DERIV-fillet-beside-curve)", () => {
  it("[F12.AC7] keeps points 2 and 4 rounded by 20 when point 3 gets handles", () => {
    // The checks table: corner 2's centre (100.15277, 20), touching the curve at (120.15232,
    // 19.86518); corner 4's (20, 60.10582), at (19.90229, 80.10558); checked against a brute
    // force: the point of y = 20 at 20 from 200 000 samples of the curve has x 100.152770476.
    const arcs = curveFillets(bentRectangle(1, [20, 20, 20, 20])).map((fillet) => fillet.arc);
    const numbers = arcs.flatMap((arc) => [
      arc.center.x,
      arc.center.y,
      arc.start.x,
      arc.start.y,
      arc.end.x,
      arc.end.y,
    ]);

    expect(arcs.map((arc) => arc.radius)).toEqual([20, 20, 0, 20]);

    // Corner 2: centre x, end x and y; corner 4: centre y, start x and y.
    expect([numbers[6], numbers[10], numbers[11], numbers[19], numbers[20], numbers[21]]).toEqual([
      expect.closeTo(100.15277, 5),
      expect.closeTo(120.15232, 5),
      expect.closeTo(19.86518, 5),
      expect.closeTo(60.10582, 5),
      expect.closeTo(19.90229, 5),
      expect.closeTo(80.10558, 5),
    ]);

    expect(numbers.slice(12, 16)).toEqual([120, 80, 120, 80]);
  });

  it("[F12.AC7] reduces a radius too large to the largest circle clear of the curve, never sharp", () => {
    // Radius 60 at point 2 beside a handle of 40, then of 80 (ADR-0047).
    const radii = [40, 80].map((handle) =>
      curveFillets(bentRectangle(handle, [20, 60, 20, 20])).map((fillet) => fillet.arc.radius),
    );

    expect(radii.map((drawn) => drawn[1])).toEqual([
      expect.closeTo(47.74907, 4),
      expect.closeTo(46.86292, 4),
    ]);
  });

  test.prop({ radii: fc.tuple(fc.nat(60), fc.nat(60), fc.nat(60), fc.nat(60)) })(
    "[F12.AC5] draws the arcs of F01 when no side is curved",
    ({ radii }) => {
      const corners = rectangleCorners(120, 80, radii);
      const handled = corners.map((corner) => ({
        ...corner,
        handles: bentRectangle(0, [0])[0]?.handles ?? {
          after: { x: 0, y: 0 },
          before: { x: 0, y: 0 },
          isOn: false,
          mode: "angle-and-length" as const,
        },
      }));

      expect(curveFillets(handled).map((fillet) => fillet.arc)).toEqual(filletArcs(corners));
    },
  );

  it("[F12.AC7] shares a straight side between a corner beside a curve and its neighbour (ADR-0007)", () => {
    // The top side, 120: radius 100 at point 1 and 60 at point 2 ask more than it holds; both are
    // reduced, their arcs meeting at most on it.
    const corners = bentRectangle(40, [100, 60, 20, 20]);
    const [first, second] = [arcAt(corners, 0), arcAt(corners, 1)];

    expect(first.end.x).toBeLessThanOrEqual(second.start.x + 1e-9);
    expect([first.radius, second.radius].every((radius) => radius > 0 && radius < 100)).toBe(true);
  });

  test.prop(
    { asked: fc.integer({ max: 80, min: 1 }), handle: fc.integer({ max: 90, min: 1 }) },
    { numRuns: 15 },
  )(
    "[F12.AC7] draws a radius that never falls as the radius asked grows, with an arc of some length",
    ({ asked, handle }) => {
      const smaller = arcAt(bentRectangle(handle, [20, asked, 20, 20]), 1);
      const larger = arcAt(bentRectangle(handle, [20, asked + 5, 20, 20]), 1);

      expect(larger.radius + 1e-6).toBeGreaterThanOrEqual(smaller.radius);

      expect(
        Math.hypot(larger.end.x - larger.start.x, larger.end.y - larger.start.y),
      ).toBeGreaterThan(0);
    },
  );
});
