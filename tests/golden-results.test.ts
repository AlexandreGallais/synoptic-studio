import { execFileSync } from "node:child_process";
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import { fc } from "@fast-check/vitest";
import { expect, it } from "vitest";

import {
  HANDLE_MODES,
  HANDLE_WAYS,
  angleFromDirection,
  contourBoundingBox,
  contourPiecesToPathData,
  curveContour,
  curveCornerOf,
  drawnRadii,
  fieldValuesOfHandle,
  freeShapeCorners,
  freeShapeWithMidpoint,
  isValidFreeShape,
  mirroredHandle,
  regularPolygonAsFreeShape,
  regularPolygonCorners,
  roundShapeAsFreeShape,
  roundShapeCorners,
  turnDirection,
  turnedCorners,
  typedHandle,
  unit,
} from "../src";

import type { CurveCorner, FreeShape, HandleFrame, TurnedBox, Vector } from "../src";

// The golden file of a refactor (ADR-0049, RET-002): about 400 results of the library — drawings,
// conversions, handles — from shapes drawn by fast-check with a fixed seed, written before the
// refactor (`npm run golden:write`) and compared bit for bit after each change
// (`npm run golden:check`). Kept in the Git directory, never committed; skipped by `npm run test`.

/** The run's mode: `golden-write`, `golden-check`, or another for the usual test run. */
const { MODE } = import.meta.env;

/** Where the golden file is kept: the Git directory, shared by every worktree. */
const GOLDEN = join(
  execFileSync("git", ["rev-parse", "--path-format=absolute", "--git-common-dir"], {
    encoding: "utf8",
  }).trim(),
  "metrics",
  "golden.json",
);

/** The same shapes on every run. */
const SAMPLE = { numRuns: 60, seed: 13 };

/** A position or a small offset. */
const OFFSET = fc.integer({ max: 50, min: -50 });

/** A size of a box, 0 included. */
const SIZE = fc.integer({ max: 300, min: 0 });

/** An angle in degrees. */
const ANGLE = fc.integer({ max: 359, min: 0 });

/** A vector of integers. */
const VECTOR = fc.record({
  x: fc.integer({ max: 80, min: -80 }),
  y: fc.integer({ max: 80, min: -80 }),
});

/**
 * What a shape draws: its path, its radii as drawn and its straight box.
 *
 * @param corners - its corners, before turning
 * @param box - its box and angle
 * @returns the drawing's results
 */
function drawing(corners: readonly CurveCorner[], box: TurnedBox): unknown {
  const turned = turnedCorners(corners, box);
  const pieces = curveContour(turned);

  return {
    box: contourBoundingBox(pieces),
    d: contourPiecesToPathData(pieces),
    radii: drawnRadii(turned),
  };
}

/**
 * Results of round shapes and their conversion.
 *
 * @returns one entry per shape
 */
function roundResults(): readonly unknown[] {
  const shapes = fc.sample(
    fc.record({
      angle: ANGLE,
      height: SIZE,
      kind: fc.constant("round" as const),
      width: SIZE,
      x: OFFSET,
      y: OFFSET,
    }),
    SAMPLE,
  );

  return shapes.map((shape) => [
    drawing(roundShapeCorners(shape), shape),
    shape.width * shape.height > 0 ? roundShapeAsFreeShape(shape) : "flat",
  ]);
}

/**
 * Results of regular polygons, stretched or not, anchored anywhere, and their conversion.
 *
 * @returns one entry per polygon
 */
function polygonResults(): readonly unknown[] {
  const place = fc.constantFrom("min" as const, "mid" as const, "max" as const);
  const polygons = fc.sample(
    fc.integer({ max: 12, min: 3 }).chain((corners) =>
      fc.record({
        anchor: fc.record({ horizontal: place, vertical: place }),
        angle: ANGLE,
        corners: fc.constant(corners),
        height: SIZE,
        radii: fc.array(fc.integer({ max: 60, min: 0 }), {
          maxLength: corners,
          minLength: corners,
        }),
        stretched: fc.boolean(),
        width: SIZE,
        x: OFFSET,
        y: OFFSET,
      }),
    ),
    SAMPLE,
  );

  return polygons.map((polygon) => [
    drawing(
      regularPolygonCorners(polygon).map((corner) => curveCornerOf(corner)),
      polygon,
    ),
    polygon.width * polygon.height > 0 ? regularPolygonAsFreeShape(polygon) : "flat",
  ]);
}

/**
 * Results of a handle typed in each way and mirrored in each mode.
 *
 * @param handle - the handle, from its point
 * @param frame - its point, its side, and whether it comes before the point
 * @returns the field values, the handle read back, the mirrors and the angle
 */
function handleResults(handle: Vector, frame: HandleFrame): unknown {
  return [
    HANDLE_WAYS.map((way) => {
      const values = fieldValuesOfHandle(way, handle, frame);

      return [
        values,
        typedHandle(way, values, frame),
        typedHandle(way, [handle.x, handle.y], frame),
      ];
    }),
    HANDLE_MODES.map((mode) => mirroredHandle(mode, handle, frame.side)),
    angleFromDirection(unit(frame.side), handle),
    turnDirection(handle.x * 9),
  ];
}

/**
 * Results of free shapes with handles: drawing, a point added on each, handles.
 *
 * @returns one entry per shape, empty for an invalid one
 */
function freeResults(): readonly unknown[] {
  const corner = fc.record({
    handles: fc.record({
      after: VECTOR,
      before: VECTOR,
      isOn: fc.boolean(),
      mode: fc.constantFrom(...HANDLE_MODES),
    }),
    point: fc.record({ x: fc.integer({ max: 200, min: 0 }), y: fc.integer({ max: 200, min: 0 }) }),
    radius: fc.integer({ max: 50, min: 0 }),
  });
  const shapes: readonly FreeShape[] = fc.sample(
    fc.record({
      angle: ANGLE,
      corners: fc.array(corner, { maxLength: 6, minLength: 3 }),
      height: fc.integer({ max: 300, min: 1 }),
      width: fc.integer({ max: 300, min: 1 }),
      x: OFFSET,
      y: OFFSET,
    }),
    { ...SAMPLE, numRuns: 80 },
  );

  return shapes.map((shape) => {
    const first = shape.corners[0];

    return first !== undefined && isValidFreeShape(shape)
      ? [
          drawing(freeShapeCorners(shape), shape),
          shape.corners
            .keys()
            .map((edge) => freeShapeWithMidpoint(shape, edge))
            .toArray(),
          handleResults(first.handles.after, {
            isBefore: first.handles.isOn,
            point: first.point,
            side: first.handles.before,
          }),
        ]
      : [];
  });
}

/**
 * A number as the golden file writes it, keeping what plain JSON loses.
 *
 * @param value - a number
 * @returns `"-0"`, `"NaN"` or `"Infinity"` for those numbers, the number otherwise
 */
function exactNumber(value: number): number | string {
  if (Object.is(value, -0)) {
    return "-0";
  }

  return Number.isFinite(value) ? value : String(value);
}

/**
 * A result as the golden file writes it, every number exact.
 *
 * @param value - any result
 * @returns the same structure, its numbers written by `exactNumber`
 */
function exactData(value: unknown): unknown {
  if (typeof value === "number") {
    return exactNumber(value);
  }

  if (typeof value !== "object" || value === null) {
    return value;
  }

  return Array.isArray(value)
    ? value.map((item: unknown) => exactData(item))
    : Object.fromEntries(Object.entries(value).map(([key, item]) => [key, exactData(item)]));
}

it.runIf(MODE === "golden-write" || MODE === "golden-check")(
  "gives the same results as the golden file, bit for bit",
  () => {
    const text = JSON.stringify(exactData([roundResults(), polygonResults(), freeResults()]));

    if (MODE === "golden-write" || !existsSync(GOLDEN)) {
      writeFileSync(GOLDEN, text);
    }

    expect(text === readFileSync(GOLDEN, "utf8")).toBe(true);
  },
  // About 10 s: every shape gets a point added on each of its sides.
  120_000,
);
