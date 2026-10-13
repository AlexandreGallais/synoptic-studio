import { describe, expect, it } from "vitest";

import { expectPoints } from "../../../tests";

import { NO_HANDLES } from "./no-handles";
import { smoothedCorners } from "./smoothed-corners";

/**
 * Three corners: (0, 0), A (100, 0) in mode auto with a tension, (200, 100).
 *
 * @param tension - A's tension
 * @returns the three corners, A in the middle
 */
function cornersAround(tension: number): ReturnType<typeof smoothedCorners> {
  return [
    { handles: NO_HANDLES, point: { x: 0, y: 0 }, radius: 0 },
    {
      handles: { ...NO_HANDLES, isOn: true, mode: "auto", tension },
      point: { x: 100, y: 0 },
      radius: 0,
    },
    { handles: NO_HANDLES, point: { x: 200, y: 100 }, radius: 0 },
  ];
}

describe("smoothedCorners (DERIV-handle-tension)", () => {
  it("[F12.AC4] computes the handles of a point in mode auto from its neighbours", () => {
    // Tension 0: ±(200, 100) / 6.
    const [, smoothed] = smoothedCorners(cornersAround(0));

    expectPoints(
      [
        smoothed?.handles.after ?? { x: NaN, y: NaN },
        smoothed?.handles.before ?? { x: NaN, y: NaN },
      ],
      [
        { x: 33.33333, y: 16.66667 },
        { x: -33.33333, y: -16.66667 },
      ],
      5,
    );
  });

  it("[F12.AC4] makes straight sides at tension 100, and follows a neighbour that moves", () => {
    expect(smoothedCorners(cornersAround(100))[1]?.handles.after).toEqual({ x: 0, y: 0 });

    const moved = cornersAround(0).map((corner, index) =>
      index === 2 ? { ...corner, point: { x: 200, y: 0 } } : corner,
    );

    expect(smoothedCorners(moved)[1]?.handles.after.x).toBeCloseTo(200 / 6, 12);
    expect(smoothedCorners(moved)[1]?.handles.after.y).toBe(0);
  });

  it("leaves the other corners as they are", () => {
    const corners = cornersAround(0);

    expect(smoothedCorners(corners)[0]).toBe(corners[0]);
  });
});
