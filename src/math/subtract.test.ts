import { test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint } from "../../tests";

import { subtract } from "./subtract";

describe("subtract", () => {
  it("subtracts component-wise: the vector from b to a", () => {
    // (100, 0) − (0, 50) = (100, −50).
    expect(subtract({ x: 100, y: 0 }, { x: 0, y: 50 })).toEqual({ x: 100, y: -50 });
  });
});

describe("subtract (properties)", () => {
  test.prop({
    a: integerPoint(1e4),
    b: integerPoint(1e4),
  })("is antisymmetric: a − b = −(b − a)", ({ a, b }) => {
    const forward = subtract(a, b);
    const backward = subtract(b, a);

    expect(forward.x + backward.x).toBeCloseTo(0, 9);
    expect(forward.y + backward.y).toBeCloseTo(0, 9);
  });
});
