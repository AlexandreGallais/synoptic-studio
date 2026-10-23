import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { transposedPiece } from "./transposed-piece";

import type { ContourPiece } from "../contour";

/** A point on integers. */
const POINT = fc.record({ x: fc.integer(), y: fc.integer() });

describe("transposedPiece (DERIV-box-overlap step 8)", () => {
  it("swaps a segment's ends", () => {
    expect(
      transposedPiece({ end: { x: 100, y: 100 }, kind: "segment", start: { x: 0, y: 20 } }),
    ).toEqual({ end: { x: 100, y: 100 }, kind: "segment", start: { x: 20, y: 0 } });
  });

  it("swaps a cubic side's four points", () => {
    // The arch (0, 0), (0, −40), (100, −40), (100, 0).
    expect(
      transposedPiece({
        end: { x: 100, y: 0 },
        first: { x: 0, y: -40 },
        kind: "cubic",
        second: { x: 100, y: -40 },
        start: { x: 0, y: 0 },
      }),
    ).toEqual({
      end: { x: 0, y: 100 },
      first: { x: -40, y: 0 },
      kind: "cubic",
      second: { x: -40, y: 100 },
      start: { x: 0, y: 0 },
    });
  });

  it("swaps an arc's ends and centre, keeps its radius and reverses its direction", () => {
    // The quarter from the left (0, 50) to the top (50, 0) of the circle about (50, 50), positive:
    // swapped, from the top (50, 0) to the left (0, 50), the angle falling from 3π/2 to π.
    expect(
      transposedPiece({
        center: { x: 50, y: 50 },
        end: { x: 50, y: 0 },
        kind: "arc",
        positive: true,
        radius: 50,
        start: { x: 0, y: 50 },
      }),
    ).toEqual({
      center: { x: 50, y: 50 },
      end: { x: 0, y: 50 },
      kind: "arc",
      positive: false,
      radius: 50,
      start: { x: 50, y: 0 },
    });
  });
});

describe("transposedPiece (properties)", () => {
  test.prop({
    center: POINT,
    end: POINT,
    first: POINT,
    kind: fc.constantFrom("arc", "cubic", "segment"),
    positive: fc.boolean(),
    radius: fc.integer({ max: 100, min: 1 }),
    second: POINT,
    start: POINT,
  })(
    "gives the piece back when applied twice",
    ({ center, end, first, kind, positive, radius, second, start }) => {
      const pieces: Readonly<Record<ContourPiece["kind"], ContourPiece>> = {
        arc: { center, end, kind: "arc", positive, radius, start },
        cubic: { end, first, kind: "cubic", second, start },
        segment: { end, kind: "segment", start },
      };

      expect(transposedPiece(transposedPiece(pieces[kind]))).toEqual(pieces[kind]);
    },
  );
});
