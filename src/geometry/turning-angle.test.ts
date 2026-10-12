import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { turningAngle } from "./turning-angle";

/** Decimals compared: atan2 of exact integers is correct to the last bits of a double. */
const DECIMALS = 12;

describe("turningAngle (DERIV-turning-angle check table)", () => {
  it("is +π/2 for a clockwise quarter turn on screen", () => {
    // a = (100, 0), b = (0, 50): perp dot 5000, dot 0.
    expect(turningAngle({ x: 0, y: 0 }, { x: 100, y: 0 }, { x: 100, y: 50 })).toBeCloseTo(
      Math.PI / 2,
      DECIMALS,
    );
  });

  it("is −π/2 for a counter-clockwise quarter turn on screen", () => {
    // a = (10, 0), b = (0, −10): perp dot −100, dot 0.
    expect(turningAngle({ x: 0, y: 0 }, { x: 10, y: 0 }, { x: 10, y: -10 })).toBeCloseTo(
      -Math.PI / 2,
      DECIMALS,
    );
  });

  it("is 0 for aligned vertices", () => {
    // a = (5, 0), b = (5, 0): perp dot 0, dot 25.
    expect(turningAngle({ x: 0, y: 0 }, { x: 5, y: 0 }, { x: 10, y: 0 })).toBeCloseTo(0, DECIMALS);
  });

  it("is π for a back-turn", () => {
    // a = (10, 0), b = (−10, 0): perp dot 0, dot −100.
    expect(Math.abs(turningAngle({ x: 0, y: 0 }, { x: 10, y: 0 }, { x: 0, y: 0 }))).toBeCloseTo(
      Math.PI,
      DECIMALS,
    );
  });

  it("is 0 when an edge has zero length", () => {
    // Repeated vertex: atan2(0, 0) = 0, no turn.
    expect(Math.abs(turningAngle({ x: 0, y: 0 }, { x: 0, y: 0 }, { x: 10, y: 0 }))).toBe(0);
  });

  it("is 0 when the outgoing edge has zero length, whatever the sign of the zeros", () => {
    // a = (−10, −10), b = (0, 0): a · b = −0 + −0 = −0 and atan2(+0, −0) = π in IEEE 754;
    // a zero-length edge has no direction, so the intended result is 0 (ADR-0025).
    expect(turningAngle({ x: 10, y: 10 }, { x: 0, y: 0 }, { x: 0, y: 0 })).toBe(0);
  });
});

describe("turningAngle (properties)", () => {
  test.prop({
    next: fc.record({
      x: fc.integer({ max: 1e4, min: -1e4 }),
      y: fc.integer({ max: 1e4, min: -1e4 }),
    }),
    offset: fc.record({
      x: fc.integer({ max: 1e4, min: -1e4 }),
      y: fc.integer({ max: 1e4, min: -1e4 }),
    }),
    previous: fc.record({
      x: fc.integer({ max: 1e4, min: -1e4 }),
      y: fc.integer({ max: 1e4, min: -1e4 }),
    }),
    vertex: fc.record({
      x: fc.integer({ max: 1e4, min: -1e4 }),
      y: fc.integer({ max: 1e4, min: -1e4 }),
    }),
  })(
    "stays in [−π, π] and does not change when the corner is moved",
    ({ next, offset, previous, vertex }) => {
      const angle = turningAngle(previous, vertex, next);
      const moved = turningAngle(
        { x: previous.x + offset.x, y: previous.y + offset.y },
        { x: vertex.x + offset.x, y: vertex.y + offset.y },
        { x: next.x + offset.x, y: next.y + offset.y },
      );

      expect(Math.abs(angle)).toBeLessThanOrEqual(Math.PI);
      expect(moved).toBeCloseTo(angle, 9);
    },
  );
});
