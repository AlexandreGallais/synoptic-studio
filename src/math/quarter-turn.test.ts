import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { dot } from "./dot";
import { norm } from "./norm";
import { perpDot } from "./perp-dot";
import { quarterTurn } from "./quarter-turn";

/** Vector with integer components. */
const VECTOR = fc.record({
  x: fc.integer({ max: 1e4, min: -1e4 }),
  y: fc.integer({ max: 1e4, min: -1e4 }),
});

describe("quarterTurn (REF-MATHWORLD-ROTATION-MATRIX, θ = π/2)", () => {
  it("maps (x, y) to (−y, x)", () => {
    // (3, 4) → (−4, 3).
    expect(quarterTurn({ x: 3, y: 4 })).toEqual({ x: -4, y: 3 });
  });

  it("turns the top edge of a clockwise rectangle towards its inside (y down)", () => {
    // (1, 0) → (−0, 1): pointing down on screen, into the rectangle.
    const turned = quarterTurn({ x: 1, y: 0 });

    expect(turned.x).toBeCloseTo(0, 12);
    expect(turned.y).toBe(1);
  });

  test.prop({ v: VECTOR })("is perpendicular and keeps the length", ({ v }) => {
    expect(dot(v, quarterTurn(v))).toBeCloseTo(0, 9);
    expect(norm(quarterTurn(v))).toBeCloseTo(norm(v), 9);
  });

  test.prop({ v: VECTOR })("turns in the positive direction of perpDot", ({ v }) => {
    // perpDot(v, v⊥) = vₓ·vₓ − v_y·(−v_y) = ‖v‖².
    expect(perpDot(v, quarterTurn(v))).toBeCloseTo(norm(v) ** 2, 6);
  });
});
