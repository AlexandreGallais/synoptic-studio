import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { regularPartition } from "./regular-partition";

describe("regularPartition (REF-OPENSTAX-REGULAR-PARTITION)", () => {
  it("divides [0, 1] into quarters and thirds", () => {
    expect(regularPartition(1, 4)).toEqual([0, 0.25, 0.5, 0.75, 1]);
    expect(regularPartition(1, 3)).toEqual([0, 0.3333333333333333, 0.6666666666666666, 1]);
  });

  it("computes each point product first: [0, 3] in tenths without drift", () => {
    // (i/n)·b, i·(b/n) and running sums of b/n give 0.30000000000000004 or 0.8999999999999999.
    expect(regularPartition(3, 10)).toEqual([0, 0.3, 0.6, 0.9, 1.2, 1.5, 1.8, 2.1, 2.4, 2.7, 3]);
  });

  it("gives [0, b] for one part and only zeros for an interval of length 0", () => {
    expect(regularPartition(7, 1)).toEqual([0, 7]);
    expect(regularPartition(0, 2)).toEqual([0, 0, 0]);
  });

  test.prop({
    count: fc.integer({ max: 6, min: 0 }),
    end: fc.double({ max: 1e6, min: 0, noNaN: true }),
  })("ends exactly at b for a power of two of parts", ({ count, end }) => {
    const parts = 2 ** count;

    expect(regularPartition(end, parts).at(-1)).toBe(end);
  });
});
