import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerTriangle, rectanglePoints } from "../../../tests";

import { cornerTriples } from "./corner-triples";
import { turningAngles } from "./turning-angles";

/** Decimals compared: atan2 of exact integers is correct to the last bits of a double. */
const DECIMALS = 12;

describe("turningAngles", () => {
  it("gives a clockwise quarter turn at every corner of the rectangle of US-001", () => {
    // The first vertex sees the last edge (0, 50) → (0, 0): cyclic contour (Q11).
    const angles = turningAngles(cornerTriples(rectanglePoints(100, 50)));

    expect(angles).toHaveLength(4);

    for (const angle of angles) {
      expect(angle).toBeCloseTo(Math.PI / 2, DECIMALS);
    }
  });

  it("gives each corner of a 3-4-5 triangle its own turn", () => {
    // (0, 0), (4, 0), (0, 3), clockwise on screen: π − the inner angles π/2, atan(3/4), atan(4/3).
    const [first, second, third] = turningAngles(
      cornerTriples([
        { x: 0, y: 0 },
        { x: 4, y: 0 },
        { x: 0, y: 3 },
      ]),
    );

    expect(first).toBeCloseTo(Math.PI / 2, DECIMALS);
    expect(second).toBeCloseTo(Math.PI - Math.atan(3 / 4), DECIMALS);
    expect(third).toBeCloseTo(Math.PI - Math.atan(4 / 3), DECIMALS);
  });

  it("gives no angle for an empty contour", () => {
    expect(turningAngles(cornerTriples([]))).toEqual([]);
  });
});

/** Triangles of positive area: their three vertices are not aligned. */
const TRIANGLE = integerTriangle(1000);

describe("turningAngles (properties)", () => {
  test.prop({
    dx: fc.integer({ max: 1000, min: -1000 }),
    dy: fc.integer({ max: 1000, min: -1000 }),
    triangle: TRIANGLE,
  })("turns a triangle by one full turn, the same wherever it is", ({ dx, dy, triangle }) => {
    // A simple closed polygon turns by ±2π in all (turning number 1).
    const angles = turningAngles(cornerTriples(triangle));
    const moved = turningAngles(
      cornerTriples(triangle.map((point) => ({ x: point.x + dx, y: point.y + dy }))),
    );

    expect(Math.abs(angles.reduce((sum, angle) => sum + angle, 0))).toBeCloseTo(2 * Math.PI, 9);

    for (const [index, angle] of moved.entries()) {
      expect(angle).toBeCloseTo(angles[index] ?? 0, 9);
    }
  });

  test.prop({ triangle: TRIANGLE })(
    "turns at each vertex by π minus the inner angle there",
    ({ triangle }) => {
      // |τᵢ| = π − acos(u · v / (|u| |v|)), u and v from vertex i to its neighbours.
      const angles = turningAngles(cornerTriples(triangle));

      for (const [index, vertex] of triangle.entries()) {
        const previous = triangle[(index + 2) % 3] ?? vertex;
        const next = triangle[(index + 1) % 3] ?? vertex;
        const u = { x: previous.x - vertex.x, y: previous.y - vertex.y };
        const v = { x: next.x - vertex.x, y: next.y - vertex.y };
        const inner = Math.acos(
          (u.x * v.x + u.y * v.y) / (Math.hypot(u.x, u.y) * Math.hypot(v.x, v.y)),
        );

        expect(Math.abs(angles[index] ?? 0)).toBeCloseTo(Math.PI - inner, 6);
      }
    },
  );
});
