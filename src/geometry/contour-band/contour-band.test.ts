import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { subtract } from "../../math";
import { polygonSegments } from "../painted-area";

import { circleArcs } from "./circle-arcs";
import { contourBand } from "./contour-band";

import type { Point } from "../../math";
import type { Arc, ContourPiece, Segment } from "../contour";

/**
 * The closed sides of a polygon.
 *
 * @param points - its corners, in order
 * @returns its sides as segments
 */
function sides(points: readonly Point[]): readonly Segment[] {
  return polygonSegments(points);
}

/**
 * The vector of a subpath's first side, from its start to its end.
 *
 * @param subpath - a band's sides
 * @returns its first side's end less its start; (0, 0) for none
 */
function sideVector(subpath: readonly ContourPiece[]): Point {
  const [side = { end: { x: 0, y: 0 }, kind: "segment", start: { x: 0, y: 0 } }] = subpath;

  return subtract(side.end, side.start);
}

describe("contourBand", () => {
  it("bands a flat shape 10 × 0 there and back: a disc and a rectangle for each side", () => {
    // Out, d = (10, 0), n = (−0, 1): (0, −1), (10, −1), (10, 1), (0, 1). Back, d = (−10, 0),
    // n = (−0, −1): (10, 1), (0, 1), (0, −1), (10, −1). A disc of radius 1 at each side's start.
    const band = contourBand(
      [
        { end: { x: 10, y: 0 }, kind: "segment", start: { x: 0, y: 0 } },
        { end: { x: 0, y: 0 }, kind: "segment", start: { x: 10, y: 0 } },
      ],
      1,
      2,
    );

    expect(band).toEqual([
      circleArcs({ x: 0, y: 0 }, 1),
      sides([
        { x: 0, y: -1 },
        { x: 10, y: -1 },
        { x: 10, y: 1 },
        { x: 0, y: 1 },
      ]),
      circleArcs({ x: 10, y: 0 }, 1),
      sides([
        { x: 10, y: 1 },
        { x: 0, y: 1 },
        { x: 0, y: -1 },
        { x: 10, y: -1 },
      ]),
    ]);
  });

  it("follows an arc by its chords: a quarter of radius 10, a step of 4, a disc and 4 bands", () => {
    // 10 · π/2 = 15.708 long, 4 chords (`pieceSamples`), each a rectangle of 4 sides.
    const arc: Arc = {
      center: { x: 0, y: 0 },
      end: { x: 0, y: 10 },
      kind: "arc",
      positive: true,
      radius: 10,
      start: { x: 10, y: 0 },
    };
    const band = contourBand([arc], 1, 4);

    expect(band.map((subpath) => subpath.length)).toEqual([4, 4, 4, 4, 4]);
    expect(band[0]).toEqual(circleArcs({ x: 10, y: 0 }, 1));

    // Each band runs along its chord, from one point to the next: (10, 0), (9.2388, 3.8268),
    // (7.0711, 7.0711), (3.8268, 9.2388), (0, 10), at π/8 apart; its first side is the chord moved.
    const points = [
      { x: 10, y: 0 },
      { x: 9.238795325, y: 3.826834324 },
      { x: 7.071067812, y: 7.071067812 },
      { x: 3.826834324, y: 9.238795325 },
      { x: 0, y: 10 },
    ];
    const chords = points.slice(1).map((to, index) => subtract(to, points[index] ?? to));
    const firstSides = band.slice(1).map((subpath) => sideVector(subpath));

    for (const [index, chord] of chords.entries()) {
      expect(firstSides[index]?.x).toBeCloseTo(chord.x, 8);
      expect(firstSides[index]?.y).toBeCloseTo(chord.y, 8);
    }
  });

  it("gives a point, a side of length 0, its disc alone", () => {
    const point = { x: 5, y: 5 };

    expect(contourBand([{ end: point, kind: "segment", start: point }], 2, 2)).toEqual([
      circleArcs(point, 2),
    ]);
  });

  it("gives none for no piece", () => {
    expect(contourBand([], 1, 2)).toEqual([]);
  });
});

describe("contourBand (properties)", () => {
  /** A point on integers of a drawing. */
  const point = fc.record({
    x: fc.integer({ max: 100, min: -100 }),
    y: fc.integer({ max: 100, min: -100 }),
  });

  test.prop({
    halfWidth: fc.integer({ max: 4, min: 1 }),
    points: fc.array(point, { maxLength: 6 }),
  })(
    "gives a polygon a disc per side and a rectangle per side of some length, each closed",
    ({ halfWidth, points }) => {
      const pieces = polygonSegments(points);
      const long = pieces.filter(
        (side) => side.start.x !== side.end.x || side.start.y !== side.end.y,
      );
      const band = contourBand(pieces, halfWidth, 2);

      expect(band).toHaveLength(pieces.length + long.length);

      for (const subpath of band) {
        expect(subpath.at(-1)?.end).toEqual(subpath[0]?.start);
      }
    },
  );
});
