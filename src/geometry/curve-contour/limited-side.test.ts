import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { LINE_AND_THIRDS } from "../../../tests";

import { limitedSide } from "./limited-side";

describe("limitedSide (ADR-0007, DERIV-fillet-beside-curve step 11)", () => {
  it("limits a straight side shared with a reduced factor to that share of the setback", () => {
    // Factor 0.5 of a setback of 40 on a side 100 long: the fillet may reach 0.2 of it.
    const [line, curve] = LINE_AND_THIRDS;

    expect(limitedSide(line, 0.5, 40).limit).toBe(0.2);
    expect(limitedSide(line, 1, 40).limit).toBe(1);
    expect(limitedSide(curve, 0.5, 40).limit).toBe(1);
  });

  test.prop({
    factor: fc.double({ max: 1, min: 0, noNaN: true }),
    setback: fc.integer({ max: 100, min: 0 }),
  })("never lets a fillet reach further than the whole side", ({ factor, setback }) => {
    expect(limitedSide(LINE_AND_THIRDS[0], factor, setback).limit).toBeLessThanOrEqual(1);
  });
});
