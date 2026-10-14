import { test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint } from "../../../tests";

import { distance } from "./distance";
import { norm } from "./norm";
import { subtract } from "./subtract";

describe("distance (REF-MATHWORLD-DISTANCE)", () => {
  it("is 5 from (0, 0) to (3, 4)", () => {
    expect(distance({ x: 0, y: 0 }, { x: 3, y: 4 })).toBe(5);
  });

  test.prop({ a: integerPoint(1e4), b: integerPoint(1e4) })(
    "is the same both ways, the norm of either difference bit for bit",
    ({ a, b }) => {
      expect(distance(a, b)).toBe(distance(b, a));
      expect(distance(a, b)).toBe(norm(subtract(a, b)));
    },
  );
});
