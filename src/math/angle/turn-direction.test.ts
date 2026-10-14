import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { quarterTurn } from "../rotation";
import { norm } from "../vector";

import { turnDirection } from "./turn-direction";

describe("turnDirection (DERIV-turned-shape step 3)", () => {
  it.each([
    { angle: 0, x: 1, y: 0 },
    // A quarter turn clockwise on screen: right becomes down.
    { angle: 90, x: 0, y: 1 },
    { angle: 180, x: -1, y: 0 },
    { angle: 270, x: 0, y: -1 },
    // −90 is kept as 270.
    { angle: -90, x: 0, y: -1 },
    { angle: 450, x: 0, y: 1 },
  ])("[F04.AC1] is exactly ($x, $y) at $angle°, without rounding dust", ({ angle, x, y }) => {
    const direction = turnDirection(angle);

    // Object.is: no 6.1e-17 of cos(π/2), and no negative zero.
    expect(Object.is(direction.x, x)).toBe(true);
    expect(Object.is(direction.y, y)).toBe(true);
  });

  it("is (√2/2, √2/2) at 45°", () => {
    const direction = turnDirection(45);

    expect(direction.x).toBeCloseTo(Math.SQRT1_2, 15);
    expect(direction.y).toBeCloseTo(Math.SQRT1_2, 15);
  });

  it("is (cos 30°, −sin 30°) at 330°: 270 then 60", () => {
    const direction = turnDirection(330);

    expect(direction.x).toBeCloseTo(Math.sqrt(3) / 2, 15);
    expect(direction.y).toBeCloseTo(-0.5, 15);
  });

  test.prop({ angle: fc.integer({ max: 1e6, min: -1e6 }) })("is a unit vector", ({ angle }) => {
    expect(norm(turnDirection(angle))).toBeCloseTo(1, 12);
  });

  test.prop({ angle: fc.integer({ max: 1e6, min: -1e6 }) })(
    "turns by a quarter turn when the angle grows by 90",
    ({ angle }) => {
      const turned = quarterTurn(turnDirection(angle));
      const next = turnDirection(angle + 90);

      expect(next.x).toBeCloseTo(turned.x, 12);
      expect(next.y).toBeCloseTo(turned.y, 12);
    },
  );
});
