import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { distance } from "../../math";
import { isContourTouchingBox } from "../box-overlap";
import { widenedBox } from "../placement";

import { circleArcs } from "./circle-arcs";
import { contourBand } from "./contour-band";
import { contourBandWithin } from "./contour-band-within";

import type { Point } from "../../math";
import type { Arc, ContourPiece, Cubic } from "../contour";
import type { BoundingBox } from "../placement";

/**
 * The box from one corner to the other.
 *
 * @param from - its top-left corner
 * @param to - its bottom-right corner
 * @returns its extreme coordinates
 */
function boxOf(from: Point, to: Point): BoundingBox {
  return { maxX: to.x, maxY: to.y, minX: from.x, minY: from.y };
}

/**
 * A quarter of a circle, turning in the positive direction of angles.
 *
 * @param center - its center
 * @param start - where it starts
 * @param end - where it ends, a quarter turn on
 * @returns the arc
 */
function quarter(center: Point, start: Point, end: Point): Arc {
  return { center, end, kind: "arc", positive: true, radius: distance(center, start), start };
}

/**
 * The point of a circle at an angle.
 *
 * @param center - where the circle turns about
 * @param radius - how far the point is from there
 * @param angle - in radians, from the x axis toward y
 * @returns its coordinates
 */
function circlePoint(center: Point, radius: number, angle: number): Point {
  return { x: center.x + radius * Math.cos(angle), y: center.y + radius * Math.sin(angle) };
}

/** R's corners' radius: half its smaller side, 99 999 / 2. */
const R_RADIUS = 49_999.5;

/**
 * The contour of R, the rectangle 99 999 × 100 000 at (0, 0) of radius 100 000: four quarters of
 * radius 49 999.5, its top and bottom sides 0 long, its left and right sides 1 long.
 */
const R_CONTOUR: readonly ContourPiece[] = [
  quarter({ x: R_RADIUS, y: R_RADIUS }, { x: 0, y: R_RADIUS }, { x: R_RADIUS, y: 0 }),
  { end: { x: R_RADIUS, y: 0 }, kind: "segment", start: { x: R_RADIUS, y: 0 } },
  quarter({ x: R_RADIUS, y: R_RADIUS }, { x: R_RADIUS, y: 0 }, { x: 99_999, y: R_RADIUS }),
  { end: { x: 99_999, y: 50_000.5 }, kind: "segment", start: { x: 99_999, y: R_RADIUS } },
  quarter({ x: R_RADIUS, y: 50_000.5 }, { x: 99_999, y: 50_000.5 }, { x: R_RADIUS, y: 100_000 }),
  { end: { x: R_RADIUS, y: 100_000 }, kind: "segment", start: { x: R_RADIUS, y: 100_000 } },
  quarter({ x: R_RADIUS, y: 50_000.5 }, { x: R_RADIUS, y: 100_000 }, { x: 0, y: 50_000.5 }),
  { end: { x: 0, y: R_RADIUS }, kind: "segment", start: { x: 0, y: 50_000.5 } },
];

/** The helps' box at the opening view, the area shown widened by half the canvas (step 1). */
const OPENING_HELPS = boxOf({ x: -640, y: -440 }, { x: 1760, y: 1160 });

/** The quarter of the circle of radius 10 about the origin, from (10, 0) down to (0, 10). */
const QUARTER = quarter({ x: 0, y: 0 }, { x: 10, y: 0 }, { x: 0, y: 10 });

/** The segment from (0, 0) to (10, 0). */
const SEGMENT: ContourPiece = { end: { x: 10, y: 0 }, kind: "segment", start: { x: 0, y: 0 } };

describe("contourBandWithin (DERIV-band-within-box)", () => {
  it("[F42.AC5] gives R's band no subpath at 100 %, its contour 18 647 units from the box", () => {
    // Its top-left corner: 39 270 chords of 2, three samples tested (DERIV-band-within-box,
    // checks); its nearest point √(48 239.5² + 48 839.5²) − 49 999.5 = 18 647.0 from (1 760,
    // 1 160); its sides' starts 48 239.5 and more away; no disc's centre within the box.
    expect(contourBandWithin(R_CONTOUR, { halfWidth: 1, step: 2 }, OPENING_HELPS)).toEqual([]);
  });

  it("follows R fitted at 0.0072 whole, each corner by 283 chords", () => {
    // Half width 1 / 0.0072, chords 2 / 0.0072 = 277.78: ⌈78 539.03 / 277.78⌉ = 283 a corner.
    // The helps' box of the fitted view, o = (−33 333.83, −5 555.56): from (o.x − 600 / 0.0072,
    // o.y − 400 / 0.0072) to (o.x + 1 800 / 0.0072, o.y + 1 200 / 0.0072), holding R.
    const size = { halfWidth: 1 / 0.0072, step: 2 / 0.0072 };
    const helps = boxOf({ x: -116_667.17, y: -61_111.12 }, { x: 216_666.17, y: 161_111.12 });
    const band = contourBandWithin(R_CONTOUR, size, helps);

    expect(band).toEqual(contourBand(R_CONTOUR, size.halfWidth, size.step));
    // 8 discs, 4 × 283 chords, the two sides 1 long one chord each, those 0 long none.
    expect(band).toHaveLength(8 + 4 * 283 + 2);
  });

  it("keeps a segment's disc and band whole for a box thinner than the band beside it", () => {
    // The box from (0, 0.2) to (0.5, 0.4), widened by 1: from (−1, −0.8) to (1.5, 1.4), holding
    // the start (0, 0) and meeting the segment: the whole band.
    expect(
      contourBandWithin(
        [SEGMENT],
        { halfWidth: 1, step: 2 },
        boxOf({ x: 0, y: 0.2 }, { x: 0.5, y: 0.4 }),
      ),
    ).toEqual(contourBand([SEGMENT], 1, 2));
  });

  it("gives a segment none for a box beyond its band", () => {
    // From (20, 0) to (30, 1), widened by 1: from (19, −1) to (31, 2), which the segment misses.
    expect(
      contourBandWithin(
        [SEGMENT],
        { halfWidth: 1, step: 2 },
        boxOf({ x: 20, y: 0 }, { x: 30, y: 1 }),
      ),
    ).toEqual([]);
  });

  it("keeps an arc tangent to the widened box by its disc and its first chord", () => {
    // ⌈15.708 / 4⌉ = 4 chords; the box from (11, −5) to (20, 5), widened by 1, from (10, −6) to
    // (21, 6): the start (10, 0) on its left side, the chord to (9.23880, 3.82683) meeting it
    // there, the others left of x = 9.24.
    const band = contourBandWithin(
      [QUARTER],
      { halfWidth: 1, step: 4 },
      boxOf({ x: 11, y: -5 }, { x: 20, y: 5 }),
    );
    const [, first] = contourBand([QUARTER], 1, 4);

    expect(band).toEqual([circleArcs({ x: 10, y: 0 }, 1), first]);
  });

  it("keeps a chord of a cubic seen through the box, not its disc", () => {
    // A straight cubic from (0, 0) to (30, 0), its control points (10, 0) and (20, 0), 3 · 10 = 30
    // over a step of 10: 3 chords, (0, 0)–(10, 0), (10, 0)–(20, 0), (20, 0)–(30, 0). The box from
    // (12, −5) to (18, 5), widened by 1, from (11, −6) to (19, 6), meets the middle one alone.
    const cubic: Cubic = {
      end: { x: 30, y: 0 },
      first: { x: 10, y: 0 },
      kind: "cubic",
      second: { x: 20, y: 0 },
      start: { x: 0, y: 0 },
    };

    // The whole band: the disc at (0, 0), then the three chords' bands.
    const middle = contourBand([cubic], 1, 10)[2];

    expect(
      contourBandWithin(
        [cubic],
        { halfWidth: 1, step: 10 },
        boxOf({ x: 12, y: -5 }, { x: 18, y: 5 }),
      ),
    ).toEqual([middle]);
  });

  it("keeps a chord meeting the widened box at its end only, the samples' rounding errors aside", () => {
    // The straight cubic from (0, 0) to (3, 0), its control points (1, 0) and (2, 0): 3 · 1 = 3
    // over a step of 0.45, ⌈6.67⌉ = 7 chords of 3 / 7. The box from (4, 0) to (7, 0), widened by 1,
    // from (3, −1) to (8, 1), meets the last chord at its end (3, 0) alone. The run from 5 to 7
    // has its middle B(6 / 7) = 2.571428571428571, a reach of 3 / 7 = 0.42857142857142855,
    // 2.9999999999999996 in all, which only `EPSILON` takes to 3.
    const cubic: Cubic = {
      end: { x: 3, y: 0 },
      first: { x: 1, y: 0 },
      kind: "cubic",
      second: { x: 2, y: 0 },
      start: { x: 0, y: 0 },
    };
    const last = contourBand([cubic], 1, 0.45)[7];

    expect(
      contourBandWithin(
        [cubic],
        { halfWidth: 1, step: 0.45 },
        boxOf({ x: 4, y: 0 }, { x: 7, y: 0 }),
      ),
    ).toEqual([last]);
  });

  it("gives a point its disc alone, and none for no piece", () => {
    const point = { x: 5, y: 5 };

    expect(
      contourBandWithin(
        [{ end: point, kind: "segment", start: point }],
        { halfWidth: 2, step: 2 },
        boxOf(point, point),
      ),
    ).toEqual([circleArcs(point, 2)]);

    expect(contourBandWithin([], { halfWidth: 1, step: 2 }, OPENING_HELPS)).toEqual([]);
  });
});

describe("contourBandWithin (properties)", () => {
  /** A point on integers of a drawing. */
  const point = fc.record({
    x: fc.integer({ max: 200, min: -200 }),
    y: fc.integer({ max: 200, min: -200 }),
  });

  /** A segment between two points on integers. */
  const segment = fc
    .tuple(point, point)
    .map(([start, end]): ContourPiece => ({ end, kind: "segment", start }));

  /** An arc of less than a half turn about a point on integers. */
  const arc = fc
    .record({
      center: point,
      from: fc.double({ max: 6.2, min: 0, noNaN: true }),
      radius: fc.integer({ max: 150, min: 1 }),
      sweep: fc.double({ max: 3, min: -3, noNaN: true }).filter((turn) => Math.abs(turn) > 0.01),
    })
    .map(({ center, from, radius, sweep }): ContourPiece => ({
      center,
      end: circlePoint(center, radius, from + sweep),
      kind: "arc",
      positive: sweep > 0,
      radius,
      start: circlePoint(center, radius, from),
    }));

  /** A cubic on four points on integers. */
  const cubic = fc
    .tuple(point, point, point, point)
    .map(([start, first, second, end]): ContourPiece => ({
      end,
      first,
      kind: "cubic",
      second,
      start,
    }));

  test.prop({
    box: fc.record({
      height: fc.integer({ max: 200, min: 0 }),
      width: fc.integer({ max: 200, min: 0 }),
      x: fc.integer({ max: 200, min: -300 }),
      y: fc.integer({ max: 200, min: -300 }),
    }),
    halfWidth: fc.integer({ max: 6, min: 1 }),
    pieces: fc.array(fc.oneof(segment, arc, cubic), { maxLength: 4 }),
    step: fc.integer({ max: 30, min: 2 }),
  })(
    "paints the box as the whole band, none of its subpaths wholly outside the box widened",
    ({ box, halfWidth, pieces, step }) => {
      const asked = boxOf({ x: box.x, y: box.y }, { x: box.x + box.width, y: box.y + box.height });
      const whole = contourBand(pieces, halfWidth, step).map((subpath) => JSON.stringify(subpath));
      const within = contourBandWithin(pieces, { halfWidth, step }, asked);
      const kept = within.map((subpath) => JSON.stringify(subpath));

      // The whole band's subpaths, in their order: each kept one found after the one before.
      const indices = kept.map((subpath) => whole.indexOf(subpath));

      expect(indices.every((index, at) => index >= 0 && index > (indices[at - 1] ?? -1))).toBe(
        true,
      );

      // Every one left out paints no point of the box: the cut band paints it as the whole one.
      for (const [index, subpath] of contourBand(pieces, halfWidth, step).entries()) {
        if (!indices.includes(index)) {
          expect(isContourTouchingBox(subpath, asked)).toBe(false);
        }
      }

      // Every one kept has a point in the box widened by the half width.
      for (const subpath of within) {
        expect(isContourTouchingBox(subpath, widenedBox(asked, halfWidth))).toBe(true);
      }
    },
  );
});
