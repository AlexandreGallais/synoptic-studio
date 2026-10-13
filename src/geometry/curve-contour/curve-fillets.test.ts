import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { bentRectangle, rectangleCorners } from "../../../tests";
import { filletArcs } from "../corner-radius";

import { curveFillets } from "./curve-fillets";

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
});
