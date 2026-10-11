import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { degreesInTurn } from "../math";

import { isShapeAngle } from "./is-shape-angle";

describe("isShapeAngle (ADR-0035)", () => {
  it.each([0, 45, 90, 359])("[F04.AC1] accepts %d", (angle) => {
    expect(isShapeAngle(angle)).toBe(true);
  });

  it.each([360, -1, -90, 1.5, NaN, Infinity])(
    "[F04.AC1] refuses %d, which is kept from 0 to 359 before it is stored",
    (angle) => {
      expect(isShapeAngle(angle)).toBe(false);
    },
  );

  test.prop({ angle: fc.integer({ max: 1e9, min: -1e9 }) })(
    "accepts every angle kept by degreesInTurn",
    ({ angle }) => {
      expect(isShapeAngle(degreesInTurn(angle))).toBe(true);
    },
  );
});
