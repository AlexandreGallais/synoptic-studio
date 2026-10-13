import { test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint } from "../../../tests";

import { dot } from "./dot";
import { norm } from "./norm";

describe("dot", () => {
  it("sums the products of the components", () => {
    // 3 × 4 + 2 × (−1) = 10.
    expect(dot({ x: 3, y: 2 }, { x: 4, y: -1 })).toBe(10);
  });

  it("is zero for perpendicular vectors", () => {
    expect(dot({ x: 100, y: 0 }, { x: 0, y: 50 })).toBe(0);
  });
});

describe("dot (properties)", () => {
  test.prop({
    a: integerPoint(1e4),
    b: integerPoint(1e4),
  })("is symmetric", ({ a, b }) => {
    expect(dot(a, b)).toBe(dot(b, a));
  });

  test.prop({
    v: integerPoint(1e4),
  })("gives the squared length of a vector with itself", ({ v }) => {
    expect(dot(v, v)).toBeCloseTo(norm(v) ** 2, 4);
  });
});
