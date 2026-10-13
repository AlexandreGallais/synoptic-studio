import { fc } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import {
  doubleSignedArea,
  expectPoints,
  integerTriangle,
  rectanglePoints,
} from "./geometry-fixtures";

describe("geometry fixtures", () => {
  it("gives the rectangle of US-001 a positive area, clockwise on screen, and its reverse a negative one", () => {
    // Shoelace sum of 100 × 50: 2 · 5000.
    expect(doubleSignedArea(rectanglePoints(100, 50))).toBe(10_000);
    expect(doubleSignedArea(rectanglePoints(100, 50).toReversed())).toBe(-10_000);
  });

  it("fails a list of points of another length or with one coordinate off", () => {
    const square = rectanglePoints(1, 1);

    expect(() => {
      expectPoints(square.slice(1), square, 9);
    }).toThrow();

    expect(() => {
      expectPoints(square, [...square.slice(0, 3), { x: 0, y: 2 }], 9);
    }).toThrow();
  });

  it("never draws three aligned points as a triangle", () => {
    // Points on one line from a generator of ±1: the filter must reject them all.
    const aligned = fc.sample(integerTriangle(1), 1000);

    expect(
      aligned.every(([a, b, c]) => (b.x - a.x) * (c.y - a.y) !== (b.y - a.y) * (c.x - a.x)),
    ).toBe(true);
  });
});
