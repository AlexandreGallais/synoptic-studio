import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { LINE_AND_HANDLE_OFF } from "../../../tests";
import { norm } from "../../math";

import { sideTangent } from "./side-tangent";

describe("sideTangent (DERIV-fillet-beside-curve step 1)", () => {
  it("leaves the corner along the curve even when its handle is off", () => {
    // B′(0) = 0 there: the direction of P₂ − P₀ = (0, 100/3), down.
    expect(sideTangent(LINE_AND_HANDLE_OFF[1].cubic, 0)).toEqual({ x: 0, y: 1 });
  });

  test.prop({ t: fc.double({ max: 1, min: 0, noNaN: true }) })("is a unit vector", ({ t }) => {
    expect(norm(sideTangent(LINE_AND_HANDLE_OFF[1].cubic, t))).toBeCloseTo(1, 12);
  });
});
